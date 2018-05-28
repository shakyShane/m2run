use context::RunContext;
use context::create_run_context;
use options::get_options_hash;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};

#[derive(Debug)]
pub struct IncomingCommand {
    pub command: &'static str,
    pub args: Vec<String>,
    pub stdin: &'static str,
    pub env: HashMap<String, String>,
    pub desc: &'static str,
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
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(cmd.stdin.as_bytes());
            match child.wait_with_output() {
                Ok(output) => {
                    if output.status.success() {
                        Ok(output.status)
                    } else {
                        Err(Error::new(ErrorKind::Other, "Nope"))
                    }
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn get_run_context() -> Result<RunContext, String> {
    has_docker()
        .and_then(|_x| get_options_hash())
        .and_then(|(cwd_as_buf, opts)| create_run_context(&cwd_as_buf, &opts))
}

fn has_docker() -> Result<ExitStatus, String> {
    match Command::new("docker")
        .stdout(Stdio::null())
        .arg("-v")
        .status()
    {
        Ok(t) => Ok(t),
        Err(_e) => Err("Docker is required".to_string()),
    }
}

pub fn current_working_dir() -> PathBuf {
    current_dir().unwrap()
}
