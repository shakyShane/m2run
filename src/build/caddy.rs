use build::CADDY_TAG_SUFFIX;
use build::{create_build_arg, create_build_tag};
use command::IncomingCommand;
use context::RunContext;
use std::collections::HashMap;

pub fn build_caddy_command(run_context: &RunContext) -> IncomingCommand {
    let caddy_build_image_text = include_str!("../templates/contrib/caddy.Dockerfile");
    let caddy_build_file_text = include_str!("../templates/contrib/Caddyfile");
    let caddy_build_tag: String = create_build_tag(&run_context.cwd_file_name, CADDY_TAG_SUFFIX);

    let caddy_build_args = vec![
        "build",
        "-",
        "-t",
        &caddy_build_tag,
        "--build-arg",
        &*create_build_arg(
            "caddyfile",
            caddy_build_file_text,
            "file:templates/Caddyfile",
            &run_context.mode,
        ),
    ].iter()
        .map(|x| x.to_string())
        .collect();

    IncomingCommand {
        command: "docker",
        args: caddy_build_args,
        stdin: caddy_build_image_text,
        env: HashMap::new(),
        desc: "Builds the Caddy image (Web server)",
    }
}
