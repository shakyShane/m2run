use std::io::Error;

use build::caddy::build_caddy_command;
use build::php::docker_build_php_command;
use command::ExecCommand;
use context::RunContext;
use context::RunMode;

mod caddy;
mod php;

const TAG_PREFIX: &'static str = "m2run";
pub const PHP_TAG_SUFFIX: &'static str = "php";
pub const CADDY_TAG_SUFFIX: &'static str = "caddy";

pub fn build_caddy(run_context: &RunContext) -> Result<ExecCommand, Error> {
    Ok(build_caddy_command(run_context))
}

pub fn build_php(run_context: &RunContext) -> Result<ExecCommand, Error> {
    Ok(docker_build_php_command(run_context))
}

fn create_build_arg(
    name: &str,
    text: &str,
    origin: &str,
    mode: &RunMode,
) -> String {
    match mode {
        &RunMode::Execute => format!("{}={}", name, text),
        &RunMode::DryRun => format!("{}={}", name, origin),
    }
}

pub fn create_build_tag(base_name: &str, suffix: &str) -> String {
    //    println!("base_name = {:?}", base_name);
    format!("{}__{}__{}", TAG_PREFIX, base_name, suffix)
}
