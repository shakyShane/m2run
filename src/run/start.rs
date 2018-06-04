use context::RunContext;
use command::IncomingCommand;
use build;
use run;

pub fn start(run_context: &RunContext) -> Vec<IncomingCommand> {
    let build_docker = build::build_php(&run_context);
    let build_caddy = build::build_caddy(&run_context);
    let run_compose = run::run(&run_context);

    let tasks = vec![build_docker.unwrap(), build_caddy.unwrap(), run_compose.unwrap()];

    tasks
}