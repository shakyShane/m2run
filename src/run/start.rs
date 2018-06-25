use context::RunContext;
use build;
use run;
use task::Task;

pub fn start(run_context: &RunContext) -> Vec<Task> {
    let build_docker = build::build_php(&run_context);
    let build_caddy = build::build_caddy(&run_context);
    let run_compose = run::run(&run_context);

    let tasks = vec![
        Task::ExecCommand(build_docker.unwrap()),
        Task::ExecCommand(build_caddy.unwrap()),
        Task::ExecCommand(run_compose.unwrap())
    ];

    tasks
}