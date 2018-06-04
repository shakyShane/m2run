use command::IncomingCommand;
use build::PHP_TAG_SUFFIX;
use context::RunContext;
use build::create_build_tag;
use std::collections::HashMap;

pub fn exec(run_context: &RunContext) -> IncomingCommand {
    let php_container_name = create_build_tag(&run_context.cwd_file_name, PHP_TAG_SUFFIX);
    let mut base = vec!["exec", "-it", "--user", &*run_context.user, &*php_container_name];

    for i in &run_context.options.trailing {
        base.push(&*i);
    }

    let docker_compose_build_args = base
        .iter()
        .map(|x| x.to_string())
        .collect();

    let env: HashMap<String, String> = HashMap::new();

    IncomingCommand {
        command: "docker",
        args: docker_compose_build_args,
        env,
        desc: "Executes a command in the PHP container",
        ..Default::default()
    }
}
