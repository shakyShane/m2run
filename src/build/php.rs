use build::PHP_TAG_SUFFIX;
use build::create_build_tag;
use command::IncomingCommand;
use context::RunContext;
use std::collections::HashMap;

pub fn docker_build_php_command(run_context: &RunContext) -> IncomingCommand {
    let docker_build_image_text = include_str!("../templates/contrib/with-deps.Dockerfile");
    let docker_build_tag: String = create_build_tag(&run_context.cwd_file_name, PHP_TAG_SUFFIX);

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
