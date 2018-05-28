use std::collections::HashMap;
use build::create_build_tag;
use command::IncomingCommand;
use context::RunContext;

const PHP_TAG_SUFFIX: &'static str = "php";

pub fn docker_build_php_command(run_context: &RunContext) -> IncomingCommand {
    let cwd_base_name = run_context
        .cwd
        .file_name()
        .expect("Could not determine base_name of directory");

    let docker_build_image_text = include_str!("../templates/contrib/with-deps.Dockerfile");
    let docker_build_tag: String = create_build_tag(&cwd_base_name, PHP_TAG_SUFFIX);

    let docker_build_args = vec![
        "build",
        "-",
        "-t",
        &docker_build_tag,
        //        "."
    ].iter()
        .map(|x| x.to_string())
        .collect();

    IncomingCommand {
        command: "docker",
        args: docker_build_args,
        stdin: docker_build_image_text,
        env: HashMap::new(),
        desc: "Builds the PHP image",
    }
}
