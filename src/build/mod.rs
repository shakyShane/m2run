use std::ffi::OsStr;
use std::io::Error;

use build::caddy::build_caddy_command;
use build::php::docker_build_php_command;
use command::IncomingCommand;
use context::RunContext;
use context::RunMode;

mod caddy;
mod php;

const TAG_PREFIX: &'static str = "m2run";

pub fn build_caddy(run_context: &RunContext) -> Result<IncomingCommand, Error> {
    Ok(build_caddy_command(run_context))
}

pub fn build_php(run_context: &RunContext) -> Result<IncomingCommand, Error> {
    Ok(docker_build_php_command(run_context))
}

fn create_build_arg(
    name: &'static str,
    text: &'static str,
    origin: &'static str,
    mode: &RunMode,
) -> String {
    match mode {
        &RunMode::Execute => format!("{}={}", name, text),
        &RunMode::DryRun => format!("{}={}", name, origin),
    }
}

fn create_build_tag(base_name: &OsStr, suffix: &'static str) -> String {
    //    println!("base_name = {:?}", base_name);
    format!(
        "{}__{}__{}",
        TAG_PREFIX,
        base_name.to_string_lossy(),
        suffix
    )
}
