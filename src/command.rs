use std::path::PathBuf;
use std::process::{Command, Stdio, Output, ExitStatus};
use std::env::set_current_dir;
use std::io::Error;
use std::io::Write;

use files::verify_files;
use std::collections::HashMap;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct IncomingCommand {
    pub command: &'static str,
    pub args: Vec<String>,
    pub stdin: &'static str,
    pub env: HashMap<String, String>
}
#[derive(Debug)]
pub struct RunContext {
    pub cwd: PathBuf,
    pub name: String
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
                },
                Err(e) => Err(e)
            }
        },
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
                            Ok(RunContext {
                                cwd: cwd.to_path_buf(),
                                name: as_string.to_string()
                            })
                        },
                        Err(e) => Err("Could not set the current working dir".to_string())
                    }
                },
                Err(e) => Err("Could not verify files".to_string())
            }
        },
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
    let mut p = PathBuf::new();
    p.push("/Users/shakyshane/Downloads/magento2-2.2-develop");
    p
}
