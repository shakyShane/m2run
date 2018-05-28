use build::{create_build_arg, create_build_tag};
use command::{IncomingCommand, RunContext};
use std::collections::HashMap;

const CADDY_TAG_SUFFIX: &'static str = "caddy";

pub fn build_caddy_command(run_context: &RunContext) -> IncomingCommand {
    let cwd_base_name = run_context
        .cwd
        .file_name()
        .expect("Could not determine base_name of directory");

    let caddy_build_image_text = include_str!("../templates/contrib/caddy.Dockerfile");
    let caddy_build_file_text = include_str!("../templates/contrib/Caddyfile");
    let caddy_build_tag: String = create_build_tag(&cwd_base_name, CADDY_TAG_SUFFIX);
    //    println!("caddy_build_tag = {}", caddy_build_tag);

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
