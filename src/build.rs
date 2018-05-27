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
    Ok(build_caddy_command(&run_context.cwd))
}

pub fn build_dockerfile(run_context: &RunContext) -> Result<IncomingCommand, Error> {
    Ok(docker_build_command(&run_context.cwd))
}

fn build_caddy_command(cwd: &PathBuf) -> IncomingCommand {
    let cwd_base_name = cwd.file_name().expect("Could not determine base_name of directory");

    let caddy_build_image_text = include_str!("templates/caddy.Dockerfile");
    let caddy_build_file_text = include_str!("templates/Caddyfile");
    let caddy_build_tag: String = create_build_tag(&cwd_base_name, CADDY_TAG_SUFFIX);
//    println!("caddy_build_tag = {}", caddy_build_tag);

    let caddy_build_args = vec![
        "build", "-",
        "-t", &caddy_build_tag,
        "--build-arg", &*format!("caddyfile={}", caddy_build_file_text),
    ].iter().map(|x| x.to_string()).collect();

    IncomingCommand {
        command: "docker",
        args: caddy_build_args,
        stdin: caddy_build_image_text,
        env: HashMap::new(),
        desc: "Builds the Caddy image (Web server)"
    }
}

fn docker_build_command(cwd: &PathBuf) -> IncomingCommand {

    let cwd_base_name = cwd.file_name().expect("Could not determine base_name of directory");

    let docker_build_image_text = include_str!("templates/Dockerfile");
    let docker_build_xdebug_text = include_str!("templates/php/xdebug.template");
    let docker_build_custom_text = include_str!("templates/php/custom.template");
    let docker_build_install_text = include_str!("templates/php/install");
    let docker_build_tag: String = create_build_tag(&cwd_base_name, PHP_TAG_SUFFIX);
//    println!("docker build, running in = {:?}", cwd);

    let docker_build_args = vec![
        "build",
        "-f", "-",
        "-t", &docker_build_tag,
        "--build-arg", &*format!("xdebug={}", docker_build_xdebug_text),
        "--build-arg", &*format!("custom={}", docker_build_custom_text),
        "--build-arg", &*format!("install={}", docker_build_install_text),
        "."
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
