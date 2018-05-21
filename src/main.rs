#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_must_use)]

use std::collections::HashMap;
use std::env;
use std::process::ExitStatus;
use std::process::Command;
use std::process::Stdio;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsString;
use std::ffi::OsStr;
use std::borrow::Cow;

// Get a users PATH, split it by ":" and print as a vector
// two approaches
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    cwd: String,
}
fn boxed_cwd() -> Box<PathBuf> {
    Box::new(std::env::current_dir().unwrap())
}
fn main() {
    run();
}


fn run() {
    match has_docker() {
        Ok(a) => {
            match std::env::current_dir() {
                Ok(p) => {
                    let file_statues = required_files_status(p);
                    {
                        let missing: Vec<&FileLookup> = file_statues.iter()
                            .filter(|x| !x.exists)
                            .collect();

                        let found: Vec<&FileLookup> = file_statues.iter()
                            .filter(|x| x.exists)
                            .collect();

                        println!("Found: {:?}", found.len());
                        println!("Missing {:?}", missing.len());
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        },
        Err(e) => println!("Docker is required")
    }
}
//
//fn build() {
//    let build_text = "FROM bluestreak/entry";
//
//    // Spawn the `wc` command
//    let mut process = match Command::new("docker")
//        .args(&["build", "-f", "-", "."])
//        .stdin(Stdio::piped())
//        .stdout(Stdio::inherit())
//        .spawn() {
//            Err(why) => panic!("couldn't spawn docker: {}", why),
//            Ok(process) => process,
//        };
//
//
//    process.stdin.as_mut().unwrap().write_all(build_text.as_bytes());
//    let output = process.wait_with_output().unwrap();
//}
//
//
//
//fn get_path_paths_as_option() -> Option<Vec<String>> {
//    let filtered_env: HashMap<String, String> = env::vars().collect();
//    match filtered_env.get("PATH") {
//        Some(t) => {
//            Some(t.split(":")
//                .map(|x| x.to_string())
//                .collect())
//        },
//        None => None
//    }
//}

//fn main() {
//
//    let ps = get_path_paths();
//    println!("{:?}", ps);

//    let docker_compose = "
//FROM
//    ";

//    match has_docker() {
//        Ok(status) => println!("Has docker!"),
//        _ => println!("{}", "Nope")
//    }
//}


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

fn required_files_status(cwd: PathBuf) -> Vec<FileLookup> {

    let required_files = vec![
        "composer.json",
        "composer.lock",
        "Cargo.toml"
    ];

    return required_files
        .into_iter()
        .map(|x| {

            let joined = Path::join(&cwd, x);

            FileLookup {
                path: x.to_string(),
                exists: joined.exists(),
                absolute: joined
            }
        })
        .collect();
}