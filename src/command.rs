use std::collections::HashMap;
use std::io::{Error, ErrorKind, Write};
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
