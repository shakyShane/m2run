use command::ExecCommand;
use context::RunContext;
use std::collections::HashMap;
use run::DOCKER_COMPOSE_TEXT;
use task::Task;

pub fn down(run_context: &RunContext) -> Task {
    let docker_compose_build_args = vec!["-f", "-", "down"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let mut env: HashMap<String, String> = HashMap::new();

    env.insert(
        "M2RUN_CONTEXT_NAME".to_string(),
        run_context.name.to_string(),
    );

    Task::ExecCommand(
        ExecCommand {
            command: "docker-compose",
            args: docker_compose_build_args,
            stdin: DOCKER_COMPOSE_TEXT,
            env,
            desc: "Stops the containers and removes them, along with networks and volumes",
        }
    )
}
