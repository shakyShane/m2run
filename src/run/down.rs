use command::IncomingCommand;
use context::RunContext;
use std::collections::HashMap;
use std::io::Error;
use run::DOCKER_COMPOSE_TEXT;

pub fn down(run_context: &RunContext) -> Result<IncomingCommand, Error> {
    let docker_compose_build_args = vec!["-f", "-", "down"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let mut env: HashMap<String, String> = HashMap::new();

    env.insert(
        "M2RUN_CONTEXT_NAME".to_string(),
        run_context.name.to_string(),
    );

    Ok(IncomingCommand {
        command: "docker-compose",
        args: docker_compose_build_args,
        stdin: DOCKER_COMPOSE_TEXT,
        env,
        desc: "Stops the containers and removes them, along with networks and volumes",
    })
}
