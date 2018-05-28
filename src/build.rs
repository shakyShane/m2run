use std::process::Stdio;
use std::process::Command;
use std::io::Write;
use std::path::PathBuf;
use std::env::set_current_dir;
use std::io::Error;
use std::ffi::OsStr;
use command;
use command::IncomingCommand;
use command::RunContext;
use command::current_working_dir;
use std::path::Path;
use std::collections::HashMap;

const TAG_PREFIX: &'static str = "m2run";
const PHP_TAG_SUFFIX: &'static str = "php";
const CADDY_TAG_SUFFIX: &'static str = "caddy";

pub fn build_caddy(run_context: &RunContext) -> Result<IncomingCommand, Error> {
    Ok(build_caddy_command(run_context))
}

pub fn build_dockerfile(run_context: &RunContext) -> Result<IncomingCommand, Error> {
    Ok(docker_build_command(run_context))
}

fn build_caddy_command(run_context: &RunContext) -> IncomingCommand {
    let cwd_base_name = run_context.cwd.file_name().expect("Could not determine base_name of directory");

    let caddy_build_image_text = include_str!("templates/caddy.Dockerfile");
    let caddy_build_file_text = include_str!("templates/Caddyfile");
    let caddy_build_tag: String = create_build_tag(&cwd_base_name, CADDY_TAG_SUFFIX);
//    println!("caddy_build_tag = {}", caddy_build_tag);

    let caddy_build_args = vec![
        "build", "-",
        "-t", &caddy_build_tag,
        "--build-arg", &*create_build_arg("caddyfile", caddy_build_file_text, "file:templates/Caddyfile", &run_context.mode),
    ].iter().map(|x| x.to_string()).collect();

    IncomingCommand {
        command: "docker",
        args: caddy_build_args,
        stdin: caddy_build_image_text,
        env: HashMap::new(),
        desc: "Builds the Caddy image (Web server)"
    }
}

fn create_build_arg(name: &'static str, text: &'static str, origin: &'static str, mode: &command::RunMode) -> String {
    match mode {
        &command::RunMode::Execute => format!("{}={}", name, text),
        &command::RunMode::DryRun => format!("{}={}", name, origin)
    }
}

fn docker_build_command(run_context: &RunContext) -> IncomingCommand {

    let cwd_base_name = run_context.cwd.file_name().expect("Could not determine base_name of directory");

    let docker_build_image_text = include_str!("templates/with-deps.Dockerfile");
//    let docker_build_xdebug_text = include_str!("templates/php/xdebug.template");
//    let docker_build_custom_text = include_str!("templates/php/custom.template");
//    let docker_build_install_text = include_str!("templates/php/install");
    let docker_build_tag: String = create_build_tag(&cwd_base_name, PHP_TAG_SUFFIX);
//    println!("docker build, running in = {:?}", cwd);

    let docker_build_args = vec![
        "build", "-",
        "-t", &docker_build_tag,
//        "--compress",
//        "--build-arg", &*create_build_arg("xdebug", docker_build_xdebug_text, "templates/php/xdebug.template", &run_context.mode),
//        "--build-arg", &*create_build_arg("custom", docker_build_custom_text, "templates/php/custom.template", &run_context.mode),
//        "--build-arg", &*create_build_arg("install", docker_build_install_text, "templates/php/install", &run_context.mode),
//        "."
    ].iter().map(|x| x.to_string()).collect();

    IncomingCommand {
        command: "docker",
        args: docker_build_args,
        stdin: docker_build_image_text,
        env: HashMap::new(),
        desc: "Builds the PHP image"
    }
}

fn create_build_tag(base_name: &OsStr, suffix: &'static str) -> String {
//    println!("base_name = {:?}", base_name);
    format!("{}__{}__{}", TAG_PREFIX, base_name.to_string_lossy(), suffix)
}