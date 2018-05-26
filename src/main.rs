#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_must_use)]

use command::get_run_context;
use command::execute_command;
use command::IncomingCommand;
use std::env;

mod build;
mod command;
mod files;
mod run;

fn main() {
    match get_run_context() {
        Ok(run_context) => {
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

                    for task in tasks {
//                        execute_command(task.unwrap());
                    }
                },
                None => println!("Please run one of the supported commands")
            }
        },
        Err(msg) => println!("{}", msg)
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
