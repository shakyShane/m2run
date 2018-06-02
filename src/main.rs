#![allow(unused_must_use)]

use command::execute_command;
use context::RunContext;
use context::RunMode;
use context::get_run_context;
use run::exec::exec;
use run::stop::stop;
use command::IncomingCommand;
use run::down::down;
use run::start::start;

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
    match select_cmd(&run_context.command) {
        Some(SubCommands::Contrib) => {
            let ts = start(&run_context).unwrap();
            sub_command_multi(&ts, &run_context)
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
            println!("{}", format_one(1, task))
        }
        RunMode::Execute => {
            execute_command(task, &run_context);
        }
    }
    Ok(())
}

fn format_one(number: usize, task: &IncomingCommand) -> String {
    format!("Task {}, Desc: {}
{}{}", number, task.desc, task.command, task
        .args
        .iter()
        .fold("".into(), |acc: String, item| acc + " " + item))
}

fn sub_command_multi(tasks: &Vec<IncomingCommand>, run_context: &RunContext) -> Result<(), String> {
    match run_context.mode {
        RunMode::DryRun => {
            let indexes = 0..tasks.len();

            for (index, task) in indexes.zip(tasks.iter()) {
                println!("{}", format_one(index, &task))
            }
        }
        RunMode::Execute => {
            for task in tasks.iter() {
                execute_command(&task, &run_context);
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
