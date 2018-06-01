use context::RunContext;
use std::io::Error;
use command::IncomingCommand;
use build;
use run;

pub fn start(run_context: &RunContext) -> Vec<Result<IncomingCommand, Error>> {
    let build_docker = build::build_php(&run_context);
    let build_caddy = build::build_caddy(&run_context);
    let run_compose = run::run(&run_context);

    let tasks = vec![build_docker, build_caddy, run_compose];

    tasks
}