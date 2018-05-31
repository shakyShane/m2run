use std::collections::HashMap;
use std::io::{Error, ErrorKind, Write};
use std::process::{Command, ExitStatus, Stdio};
use std::process::Output;
use context::RunContext;

#[derive(Debug)]
pub struct IncomingCommand<'a> {
    pub command: &'a str,
    pub args: Vec<String>,
    pub stdin: &'a str,
    pub env: HashMap<String, String>,
    pub desc: &'a str,
}

enum CommandType {
    Stdin,
    NoStdin
}

pub fn execute_command(cmd: &IncomingCommand, run_context: &RunContext) -> Result<ExitStatus, Error> {

    // is there any stdin data?
    let cmd_type = match cmd.stdin.len() {
        0 => CommandType::NoStdin,
        _ => CommandType::Stdin
    };

    let stdin_type = match cmd_type {
        CommandType::Stdin => Stdio::piped(),
        _ => Stdio::inherit()
    };

//    let merged_env: &HashMap<String, String> = run_context.env.into_iter().chain(cmd.env);
    let mut new_map: HashMap<String, String> = HashMap::new();
    for (ref k, ref v) in &run_context.env {
        println!("k={}, v={}", k, v);
        new_map.insert(k.to_string(), v.to_string());
    }
    for (ref k, ref v) in &cmd.env {
        println!("{}, {}", k, v);
        new_map.insert(k.to_string(), v.to_string());
    }

    let process = Command::new(cmd.command)
        .args(&cmd.args)
        .envs(&new_map)
        .stdin(stdin_type)
        .stdout(Stdio::inherit())
        .spawn();

    match cmd_type {
        CommandType::Stdin => {
            process
                .and_then(|mut child| {
                    child
                        .stdin
                        .as_mut()
                        .unwrap()
                        .write_all(cmd.stdin.as_bytes());
                    child.wait_with_output()
                })
                .and_then(check_output)
        },
        CommandType::NoStdin => {
            process
                .and_then(|child| child.wait_with_output())
                .and_then(check_output)
        }
    }
}

fn check_output(output: Output) -> Result<ExitStatus, Error> {
    if output.status.success() {
        Ok(output.status)
    } else {
        Err(Error::new(ErrorKind::Other, "Nope"))
    }
}
