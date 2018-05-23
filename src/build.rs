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
    let caddy_build_tag: String = create_build_tag(&cwd_base_name, CADDY_TAG_SUFFIX);
    println!("caddy_build_tag = {}", caddy_build_tag);

    let caddy_build_args = vec![
        "build", "-",
        "-t", &caddy_build_tag,
    ].iter().map(|x| x.to_string()).collect();

    IncomingCommand {
        command: "docker",
        args: caddy_build_args,
        stdin: caddy_build_image_text,
    }
}

#[test]
fn build_caddy_command_test() {
    let cwd = PathBuf::from("/Users/shakyshane/Sites/jh/graham-and-green");
    let cmd = build_caddy_command(&cwd);

    assert_eq!(cmd.command, "docker");
    assert_eq!(cmd.args, vec![
        "build",
        "-",
        "-t", "m2run__graham-and-green__caddy",
    ]);
}

fn docker_build_command(cwd: &PathBuf) -> IncomingCommand {

    let cwd_base_name = cwd.file_name().expect("Could not determine base_name of directory");

    let docker_build_image_text = include_str!("templates/Dockerfile");
    let docker_build_tag: String = create_build_tag(&cwd_base_name, PHP_TAG_SUFFIX);
    println!("docker_build_tag = {}", docker_build_tag);

    let docker_build_args = vec![
        "build",
        "-f", "-",
        "-t", &docker_build_tag,
        "."
    ].iter().map(|x| x.to_string()).collect();

    IncomingCommand {
        command: "docker",
        args: docker_build_args,
        stdin: docker_build_image_text,
    }
}

#[test]
fn docker_build_command_test() {
    let cwd = PathBuf::from("/Users/shakyshane/Sites/jh/graham-and-green");
    let cmd = docker_build_command(&cwd);

    assert_eq!(cmd.command, "docker");
    assert_eq!(cmd.args, vec![
        "build",
        "-",
        "-t", "m2run__graham-and-green__php",
        "."
    ]);
}

fn create_build_tag(base_name: &OsStr, suffix: &'static str) -> String {
    println!("base_name = {:?}", base_name);
    format!("{}__{}__{}", TAG_PREFIX, base_name.to_string_lossy(), suffix)
}
