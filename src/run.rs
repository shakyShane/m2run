use command::RunContext;
use command::IncomingCommand;
use std::collections::HashMap;
use std::io::Error;

pub fn run(run_context: &RunContext) -> Result<IncomingCommand, Error> {
    let docker_compose_text = include_str!("templates/docker-compose.yml");

    let docker_compose_build_args = vec![
        "-f", "-",
        "up",
        "-d"
    ].iter().map(|x| x.to_string()).collect();

    let mut env: HashMap<String, String> = HashMap::new();

    env.insert("M2RUN_CONTEXT_NAME".to_string(), run_context.name.to_string());

    Ok(IncomingCommand {
        command: "docker-compose",
        args: docker_compose_build_args,
        stdin: docker_compose_text,
        env: env
    })
}
