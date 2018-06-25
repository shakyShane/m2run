use command::ExecCommand;
use context::RunContext;
use std::collections::HashMap;

pub mod exec;
pub mod stop;
pub mod down;
pub mod start;

pub const DOCKER_COMPOSE_TEXT: &'static str = include_str!("../templates/contrib/docker-compose.yml");

pub fn run(_run_context: &RunContext) -> ExecCommand {
    let docker_compose_build_args = vec!["-f", "-", "up", "-d"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let env: HashMap<String, String> = HashMap::new();

    ExecCommand {
        command: "docker-compose",
        args: docker_compose_build_args,
        stdin: DOCKER_COMPOSE_TEXT,
        env,
        desc: "Runs the Application with docker-compose",
    }
}
