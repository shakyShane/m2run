#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_must_use)]

use command::get_run_context;
use command::execute_command;
use command::IncomingCommand;
use std::env;
use command::RunContext;
use command::RunMode;

mod build;
mod command;
mod files;
mod run;

fn main() {
    match get_run_context() {
        Ok(run_context) => {
            match try_to_execute(run_context) {
                Ok(x) => {},
                Err(msg) => println!("Could not run. \nReason: {}", msg),
            }
        },
        Err(msg) => println!("Could not create the Run Context. \nReason: {}", msg)
    }
}

fn try_to_execute(run_context: RunContext) -> Result<(), String> {
    match select_cmd(run_context.command.to_string()) {
        Some(SubCommands::Contrib) => {
            let build_docker = build::build_dockerfile(&run_context);
            let build_caddy = build::build_caddy(&run_context);
            let run_compose = run::run(&run_context);

            let tasks = vec![
                build_docker,
                build_caddy,
                run_compose
            ];

            match run_context.mode {
                RunMode::DryRun => {
                    let indexes = 0..tasks.len();
                    for (index, task) in indexes.zip(tasks) {
                        let unwrapped = task.unwrap();
                        println!("-------");
                        println!("Task: {}, Desc: {}",
                                 index + 1,
                                 unwrapped.desc
                        );
                        println!("-------");
                        println!("{}{}",
                                 unwrapped.command,
                                 unwrapped.args
                                     .iter()
                                     .fold("".into(), |acc: String, item| acc + " " + item)
                        );
                    }
                },
                RunMode::Execute => {
                    for task in tasks {
                        execute_command(task.unwrap());
                    }
                }
            };

            Ok(())
        },
        None => Err("Please run one of the supported commands".to_string())
    }
}

#[derive(Debug, PartialEq)]
enum SubCommands {
    Contrib,
}

fn select_cmd(maybe_cmd: String) -> Option<SubCommands> {
     match &*maybe_cmd {
         "contrib" | "c" => Some(SubCommands::Contrib),
         _ => None
     }
}

#[test]
fn select_cmd_contrib_test() {
    let res = select_cmd("contrib".to_string());
    let expected = Some(SubCommands::Contrib);
    assert_eq!(res, expected);
}
#[test]
fn select_cmd_contrib_short_test() {
    let res = select_cmd("c".to_string());
    let expected = Some(SubCommands::Contrib);
    assert_eq!(res, expected);
}
