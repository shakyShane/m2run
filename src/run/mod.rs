use command::IncomingCommand;
use context::RunContext;
use std::collections::HashMap;
use std::io::Error;
use build::create_build_tag;
use build::PHP_TAG_SUFFIX;

const DOCKER_COMPOSE_TEXT: &'static str = include_str!("../templates/contrib/docker-compose.yml");

pub fn run(run_context: &RunContext) -> Result<IncomingCommand, Error> {

    let docker_compose_build_args = vec!["-f", "-", "up", "-d"]
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
        desc: "Runs the Application with docker-compose",
    })
}

pub fn exec(run_context: &RunContext) -> Result<IncomingCommand, Error> {
    let php_container_name = create_build_tag(&run_context.cwd_file_name, PHP_TAG_SUFFIX);
    let docker_compose_build_args = vec!["-f", "-", "exec", &*php_container_name]
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
        desc: "Executes a command in the PHP container",
    })
}
