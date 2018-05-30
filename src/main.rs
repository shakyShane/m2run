#![allow(unused_must_use)]

use command::execute_command;
use context::RunContext;
use context::RunMode;
use context::get_run_context;
use run::exec::exec;
use run::stop::stop;
use command::IncomingCommand;
use run::down::down;

mod build;
mod command;
mod context;
mod files;
mod options;
mod run;

fn main() {
    match get_run_context() {
        Ok(run_context) => match try_to_execute(run_context) {
            Ok(_x) => {}
            Err(msg) => println!("Could not run. \nReason: {}", msg),
        },
        Err(msg) => println!("Could not create the Run Context. \nReason: {}", msg),
    }
}

fn try_to_execute(run_context: RunContext) -> Result<(), String> {
    match select_cmd(run_context.command.to_string()) {
        Some(SubCommands::Contrib) => {
            let build_docker = build::build_php(&run_context);
            let build_caddy = build::build_caddy(&run_context);
            let run_compose = run::run(&run_context);

            let tasks = vec![build_docker, build_caddy, run_compose];

            match run_context.mode {
                RunMode::DryRun => {
                    let indexes = 0..tasks.len();
                    for (index, task) in indexes.zip(tasks) {
                        let unwrapped = task.unwrap();
                        println!("-------");
                        println!("Task: {}, Desc: {}", index + 1, unwrapped.desc);
                        println!(
                            "{}{}",
                            unwrapped.command,
                            unwrapped
                                .args
                                .iter()
                                .fold("".into(), |acc: String, item| acc + " " + item)
                        );
                    }
                }
                RunMode::Execute => {
                    for task in tasks {
                        execute_command(&task.unwrap());
                    }
                }
            };

            Ok(())
        }
        Some(SubCommands::Exec) => {
            let task = exec(&run_context).unwrap();
            sub_command(&task, &run_context)
        }
        Some(SubCommands::Stop) => {
            let task = stop(&run_context).unwrap();
            sub_command(&task, &run_context)
        }
        Some(SubCommands::Down) => {
            let task = down(&run_context).unwrap();
            sub_command(&task, &run_context)
        }
        None => Err("Please run one of the supported commands".to_string()),
    }
}

fn sub_command(task: &IncomingCommand, run_context: &RunContext) -> Result<(), String> {
    match run_context.mode {
        RunMode::DryRun => {
            println!("-------");
            println!("Task: 1, Desc: {}", task.desc);
            println!(
                "{}{}",
                task.command,
                task.args
                    .iter()
                    .fold("".into(), |acc: String, item| acc + " " + item)
            );
        }
        RunMode::Execute => {
            execute_command(task);
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
enum SubCommands {
    Contrib,
    Exec,
    Stop,
    Down,
}

fn select_cmd(maybe_cmd: String) -> Option<SubCommands> {
    match &*maybe_cmd {
        "contrib" | "c" | "up" | "start" => Some(SubCommands::Contrib),
        "execute" | "exec" | "e" => Some(SubCommands::Exec),
        "stop" => Some(SubCommands::Stop),
        "down" | "d" => Some(SubCommands::Down),
        _ => None,
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
