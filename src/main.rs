#![allow(unused_must_use)]

extern crate core;

use command::execute_command;
use context::RunContext;
use context::RunMode;
use context::get_run_context;
use run::exec::exec;
use run::stop::stop;
use command::ExecCommand;
use run::down::down;
use run::start::start;
use print_error::print_error;

mod build;
mod command;
mod context;
mod files;
mod options;
mod run;
mod flags;
mod print_error;

fn main() {
    match get_run_context() {
        Ok(run_context) => match try_to_execute(run_context) {
            Ok(_x) => {
                /* */
            }
            Err(msg) => println!("Could not run. \nReason: {:?}", msg),
        },
        Err(err) => {
            println!("Could not create the run context, the reason was:");
            print_error(err);
        },
    }
}

fn try_to_execute(run_context: RunContext) -> Result<(), String> {
    match select_cmd(&run_context.command) {
        Some(SubCommands::Contrib) => {
            let tasks = start(&run_context);
            sub_command_multi(&tasks, &run_context)
        }
        Some(SubCommands::Exec) => {
            let task = exec(&run_context);
            sub_command(&task, &run_context)
        }
        Some(SubCommands::Stop) => {
            let task = stop(&run_context);
            sub_command(&task, &run_context)
        }
        Some(SubCommands::Down) => {
            let task = down(&run_context);
            sub_command(&task, &run_context)
        }
        None => Err("Please run one of the supported commands".to_string()),
    }
}

fn sub_command(task: &ExecCommand, run_context: &RunContext) -> Result<(), String> {
    match run_context.mode {
        RunMode::DryRun => {
            println!("\nTask: {}\n{}", 1, task)
        }
        RunMode::Execute => {
            match execute_command(task, &run_context) {
                Ok(_output) => {
                    /* the command exited successfully */
                },
                Err(_e) => println!("The following command returned a non-zero exit code:\n{}", task)
            }
        }
    }
    Ok(())
}

fn sub_command_multi(tasks: &Vec<ExecCommand>, run_context: &RunContext) -> Result<(), String> {
    match run_context.mode {
        RunMode::DryRun => {
            tasks.iter().enumerate().for_each(|(i, task)| {
                println!("\nTask: {}\n{}", i + 1, task)
            })
        }
        RunMode::Execute => {
            for task in tasks.iter() {
                execute_command(task, &run_context);
            }
        }
    };

    Ok(())
}

#[derive(Debug, PartialEq)]
enum SubCommands {
    Contrib,
    Exec,
    Stop,
    Down,
}

fn select_cmd(maybe_cmd: &Option<String>) -> Option<SubCommands> {
    match *maybe_cmd {
        Some(ref cmd_as_string) => {
            match &**cmd_as_string {
                "contrib" | "c" | "up" | "start" => Some(SubCommands::Contrib),
                "execute" | "exec" | "e" => Some(SubCommands::Exec),
                "stop" => Some(SubCommands::Stop),
                "down" | "d" => Some(SubCommands::Down),
                _ => None,
            }
        },
        None => None
    }
}

#[test]
fn select_cmd_contrib_test() {
    let res = select_cmd(&Some("contrib".into()));
    let expected = Some(SubCommands::Contrib);
    assert_eq!(res, expected);
}
#[test]
fn select_cmd_contrib_short_test() {
    let res = select_cmd(&Some("c".into()));
    let expected = Some(SubCommands::Contrib);
    assert_eq!(res, expected);
}
