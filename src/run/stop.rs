use command::IncomingCommand;
use context::RunContext;
use std::collections::HashMap;
use std::io::Error;
use run::DOCKER_COMPOSE_TEXT;

pub fn stop(_run_context: &RunContext) -> Result<IncomingCommand, Error> {
    let docker_compose_build_args = vec!["-f", "-", "stop"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let env: HashMap<String, String> = HashMap::new();

    Ok(IncomingCommand {
        command: "docker-compose",
        args: docker_compose_build_args,
        stdin: DOCKER_COMPOSE_TEXT,
        env,
        desc: "Stops the containers without losing information",
    })
}
