use std::path::PathBuf;
use std::process::{Command, Stdio, Output, ExitStatus};
use std::env::set_current_dir;
use std::env::current_dir;
use std::io::Error;
use std::io::Write;

use files::verify_files;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::env;

#[derive(Debug)]
pub struct IncomingCommand {
    pub command: &'static str,
    pub args: Vec<String>,
    pub stdin: &'static str,
    pub env: HashMap<String, String>,
}

#[derive(Debug)]
pub struct RunContext {
    pub cwd: PathBuf,
    pub name: String,
    pub command: String,
    pub opts: HashMap<String, String>,
}

pub fn execute_command(cmd: IncomingCommand) -> Result<ExitStatus, Error> {
    let process = Command::new(cmd.command)
        .args(&cmd.args)
        .envs(&cmd.env)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn();

    match process {
        Ok(mut child) => {
            child.stdin.as_mut().unwrap().write_all(cmd.stdin.as_bytes());
            match child.wait_with_output() {
                Ok(output) => {
                    if output.status.success() {
                        Ok(output.status)
                    } else {
                        Err(Error::new(ErrorKind::Other, "Nope"))
                    }
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}

pub fn get_run_context() -> Result<RunContext, String> {
    match has_docker() {
        Ok(a) => {
            let cwd = current_working_dir();
            match verify_files(&cwd) {
                Ok(num) => {
                    match set_current_dir(&cwd) {
                        Ok(_) => {
                            let context_name = cwd.file_name().unwrap();
                            let as_string = context_name.to_string_lossy();
                            let cmd = env::args().nth(1).or(Some("contrib".to_string())).unwrap();
                            let raw_opts: Vec<String> = env::args().collect();
                            let opts = &raw_opts[2..];
                            let mut defaults = HashMap::new();
                            let parsed_options = create_options_hash(opts.to_vec(), defaults);
                            Ok(RunContext {
                                cwd: cwd.to_path_buf(),
                                name: as_string.to_string(),
                                command: cmd,
                                opts: parsed_options,
                            })
                        }
                        Err(e) => Err("Could not set the current working dir".to_string())
                    }
                }
                Err(e) => Err("Could not verify files".to_string())
            }
        }
        Err(e) => Err("Docker is required".to_string())
    }
}

fn has_docker() -> Result<ExitStatus, Error> {
    Command::new("docker")
        .stdout(Stdio::null())
        .arg("-v")
        .status()
}

pub fn current_working_dir() -> PathBuf {
    current_dir().unwrap()
}

fn create_options_hash(opts: Vec<String>, defaults: HashMap<String, String>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for (key, val) in defaults.iter() {

        let flag = format!("--{}", key);
        let flag_equal = format!("--{}=", key);

        let indexes = 0..opts.len();
        let iter = indexes.zip(opts.iter());

        let matches: Vec<(usize, &String)> = iter
            .filter(|(index, opt)| flag == *opt.as_str())
            .collect();

        match matches.get(0) {
            Some((index, opt)) => {
                if let Some(next) = opts.get(index + 1) {
                    map.insert(key.to_string(), next.to_string());
                }
            },
            None => {
                map.insert(key.to_string(), val.to_string());
            }
        }
    }

    map
}

#[test]
fn create_options_hash_test() {
    let mut defaults = HashMap::new();
    defaults.insert("cwd".to_string(), "/Users/shane/Downloads".to_string());
    defaults.insert("name".to_string(), "kittie".to_string());
    let opts = vec![
        "--cwd", "/users/shane", // exists in defaults, but is overridden
        "--dc", "another" //"doesnt exist in defaults"
    ].iter().map(|x| x.to_string()).collect();
    let m = create_options_hash(opts, defaults);
    assert_eq!(m.get("cwd"), Some(&"/users/shane".to_string()));
    assert_eq!(m.get("name"), Some(&"kittie".to_string()));
    assert_eq!(m.get("dc"), None);
}

