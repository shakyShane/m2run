use command::IncomingCommand;
use context::RunContext;
use std::collections::HashMap;
use std::io::Error;

pub mod exec;
pub mod stop;
pub mod down;
pub mod start;

pub const DOCKER_COMPOSE_TEXT: &'static str = include_str!("../templates/contrib/docker-compose.yml");

pub fn run(_run_context: &RunContext) -> Result<IncomingCommand, Error> {
    let docker_compose_build_args = vec!["-f", "-", "up", "-d"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let env: HashMap<String, String> = HashMap::new();

    Ok(IncomingCommand {
        command: "docker-compose",
        args: docker_compose_build_args,
        stdin: DOCKER_COMPOSE_TEXT,
        env,
        desc: "Runs the Application with docker-compose",
    })
}
