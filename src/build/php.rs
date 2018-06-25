use build::PHP_TAG_SUFFIX;
use build::create_build_tag;
use command::ExecCommand;
use context::RunContext;
use std::collections::HashMap;

pub fn docker_build_php_command(run_context: &RunContext) -> ExecCommand {
    let docker_build_image_text = include_str!("../templates/contrib/Dockerfile");
    let docker_build_tag: String = create_build_tag(&run_context.cwd_file_name, PHP_TAG_SUFFIX);

    let docker_build_args = vec![
        "build",
        "-f",
        "-",
        "-t",
        &docker_build_tag,
        "."
    ].iter()
        .map(|x| x.to_string())
        .collect();

    ExecCommand {
        command: "docker",
        args: docker_build_args,
        stdin: docker_build_image_text,
        env: HashMap::new(),
        desc: "Builds the PHP image",
    }
}
