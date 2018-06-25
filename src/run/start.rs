use context::RunContext;
use build;
use run;
use task::Task;
use build::php::docker_ignore_write;
use build::php::docker_ignore_remove;

pub fn start(run_context: &RunContext) -> Vec<Task> {
    let build_docker = build::php::docker_build_php_command(&run_context);
    let build_caddy = build::caddy::build_caddy_command(&run_context);
    let run_compose = run::run(&run_context);

    let tasks = vec![
        Task::FileOperation(docker_ignore_write(&run_context)),
        Task::ExecCommand(build_docker),
        Task::FileOperation(docker_ignore_remove(&run_context)),
        Task::ExecCommand(build_caddy),
        Task::ExecCommand(run_compose)
    ];

    tasks
}