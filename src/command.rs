use std::path::PathBuf;
use std::process::{Command, Stdio, Output, ExitStatus};
use std::env::set_current_dir;
use std::io::Error;
use std::io::Write;

use files::verify_files;

#[derive(Debug)]
pub struct IncomingCommand {
    pub command: &'static str,
    pub args: Vec<String>,
    pub stdin: &'static str
}
#[derive(Debug)]
pub struct RunContext {
    pub cwd: PathBuf
}
pub fn execute_command(cmd: IncomingCommand) -> Result<Output, String> {
    let process = Command::new(cmd.command)
        .args(&cmd.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn();

    match process {
        Ok(mut child) => {
            child.stdin.as_mut().unwrap().write_all(cmd.stdin.as_bytes());
            let output = child.wait_with_output().unwrap();
            Ok(output)
        },
        Err(e) => {
            Err(format!("Error running {} {:?}", cmd.command, cmd.args))
        }
    }
}

pub fn get_run_context() -> Result<RunContext, String> {
    match has_docker() {
        Ok(a) => {
            let cwd = current_working_dir();
            match verify_files(&cwd) {
                Ok(num) => {
                    match set_current_dir(&cwd) {
                        Ok(_) => Ok(RunContext { cwd: cwd }),
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
    p.push("/Users/shakyshane/Sites/jh/graham-and-green");
    p
}