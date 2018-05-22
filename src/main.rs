#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_must_use)]

use std::collections::HashMap;
use std::env;
use std::process::ExitStatus;
use std::process::Command;
use std::process::Stdio;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsString;
use std::ffi::OsStr;
use std::borrow::Cow;

mod build;

fn main() {
    run();
}

fn run() {
    match has_docker() {
        Ok(a) => {
            let cwd = current_working_dir();
            match verify_files(cwd) {
                Ok(_) => {
                    println!("All files exist");
                },
                Err(_) => {
                    println!("error")
                }
            }
//            match std::env::current_dir() {
//                Ok(cwd) => {
//                    let mut p = PathBuf::new();
//                    p.push("/Users/shakyshane/Sites/jh/graham-and-green");
//
//                }
//                Err(e) => {
//                    println!("{}", "Could not determine cwd");
//                }
//            }
        },
        Err(e) => println!("Docker is required")
    }
}

fn current_working_dir() -> PathBuf {
    let mut p = PathBuf::new();
    p.push("/Users/shakyshane/Sites/jh/graham-and-green");
    p
}

fn verify_files(cwd: PathBuf) -> Result<usize, usize> {
    let required_files = vec![
        "composer.json",
        "composer.lock",
//        "app/etc/config.php",
    ];
    let file_statues = required_files_status(&required_files, &cwd);
    let (found, missing): (Vec<&FileLookup>, Vec<&FileLookup>) = file_statues
        .iter()
        .partition(|x| x.exists);

    match missing.len() {
        0 => Ok(required_files.len()),
        _num => {
            println!("Cannot continue since the following {} file(s) are missing:", _num);
            missing.iter().for_each(|x| println!("---> {}", x.path));
            println!("cwd: {:?}", cwd);
            Err(required_files.len())
        }
    }
}

fn has_docker() -> Result<ExitStatus, std::io::Error> {
    Command::new("docker")
        .stdout(Stdio::null())
        .arg("-v")
        .status()
}
#[derive(Debug)]
struct FileLookup {
    path: String,
    exists: bool,
    absolute: PathBuf,
}

fn required_files_status(files: &Vec<&str>, cwd: &PathBuf) -> Vec<FileLookup> {

    return files
        .into_iter()
        .map(|relative| (relative, Path::join(cwd, relative)))
        .map(|(relative, absolute)| {
            FileLookup {
                path: relative.to_string(),
                exists: absolute.exists(),
                absolute
            }
        })
        .collect();
}