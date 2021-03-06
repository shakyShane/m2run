use command::ExecCommand;
use context::RunContext;
use std::collections::HashMap;
use run::DOCKER_COMPOSE_TEXT;
use task::Task;

pub fn stop(_run_context: &RunContext) -> Task {
    let docker_compose_build_args = vec!["-f", "-", "stop"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let env: HashMap<String, String> = HashMap::new();

    Task::ExecCommand(ExecCommand {
        command: "docker-compose",
        args: docker_compose_build_args,
        stdin: DOCKER_COMPOSE_TEXT,
        env,
        desc: "Stops the containers without losing information",
    })
}
