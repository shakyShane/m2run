#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_must_use)]

use command::get_run_context;
use command::execute_command;

mod build;
mod command;
mod files;
mod run;

#[derive(Debug)]
enum SubCommands {
    Default,
}

fn main() {
    let cmd_to_run = SubCommands::Default;
    match get_run_context() {
        Ok(run_context) => {
            match cmd_to_run {
                SubCommands::Default => {
                    let cm_1 = build::build_dockerfile(&run_context);
                    let cm_2 = build::build_caddy(&run_context);
                    let cm_3 = run::run();
                    execute_command(cm_1.unwrap())
                        .and_then(|r| execute_command(cm_2.unwrap()))
                        .and_then(|r| execute_command(cm_3.unwrap()));
                }
            }
        },
        Err(msg) => println!("{}", msg)
    }
}