use files::verify_files;
use options;
use options::Options;
use options::get_options;
use std::env;
use std::env::current_dir;
use std::env::set_current_dir;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use std::collections::HashMap;
use std::env::Args;

#[derive(Debug)]
pub struct RunContext {
    pub cwd: PathBuf,
    pub env: HashMap<String, String>,
    pub cwd_file_name: String,
    pub name: String,
    pub user: String,
    pub command: String,
    pub options: options::Options,
    pub mode: RunMode,
}

#[derive(Debug)]
pub enum RunMode {
    DryRun,
    Execute,
}

pub const M2RUN_CONTEXT_NAME: &'static str = "M2RUN_CONTEXT_NAME";

pub fn get_run_context() -> Result<RunContext, String> {
    has_docker()
        .and_then(|_x| get_options())
        .and_then(|options: Options| {
            is_valid_dir(&options.cwd)
                .and_then(verify_files)
                .and_then(set_working_dir)
                .and_then(|_| create_run_context(options, env::args()))
        })
}

pub fn create_run_context(options: options::Options, mut args: Args) -> Result<RunContext, String> {

    let ctx_name = get_context_name(&options);
    let cmd  = select_cmd(args.nth(1));
    let mode = select_mode(options.flags.get("run_mode"));
    let user = select_user(options.flags.get("user"));
    let default_env = get_default_env(&ctx_name);

    Ok(RunContext {
        cwd: options.cwd.to_path_buf(),
        name: ctx_name.to_string(),
        command: cmd,
        cwd_file_name: ctx_name.to_string(),
        options,
        mode,
        env: default_env,
        user: user.to_string()
    })
}

fn get_context_name(options: &options::Options) -> String {
    let cwd_as_buf = options.cwd.to_path_buf();
    let context_name = cwd_as_buf.file_name().unwrap();
    let context_name_as_string = context_name.to_string_lossy().to_string();
    context_name_as_string
}

fn get_default_env(name: &String) -> HashMap<String, String> {
    let mut env: HashMap<String, String> = HashMap::new();

    env.insert(
        M2RUN_CONTEXT_NAME.to_string(),
        name.to_string(),
    );

    env
}

fn select_mode(set_mode: Option<&String>) -> RunMode {
    match set_mode {
        Some(mode) => match mode.as_str() {
            "execute" | "exe" => RunMode::Execute,
            "dry_run" | "dryrun" | "dryRun" => RunMode::DryRun,
            _ => RunMode::Execute,
        },
        None => RunMode::Execute,
    }
}

fn select_user<'a>(set_user: Option<&String>) -> &'a str {
    match set_user {
        Some(user) => match user.as_str() {
            "root" | "r" => "root",
            _ => "www-data"
        },
        None => "www-data"
    }
}

fn select_cmd(maybe_command: Option<String>) -> String {
    maybe_command.or(Some("contrib".to_string())).unwrap()
}

fn set_working_dir(path_buf: &PathBuf) -> Result<(), String> {
    match set_current_dir(&path_buf) {
        Ok(_p) => Ok(()),
        Err(_e) => Err("Could not set the current working dir".to_string()),
    }
}

fn is_valid_dir(path: &PathBuf) -> Result<&PathBuf, String> {
    if path.is_dir() {
        return Ok(path);
    }
    return Err(format!("Directory does not exist\nInput: {:?}", path));
}

fn has_docker() -> Result<ExitStatus, String> {
    match Command::new("docker")
        .stdout(Stdio::null())
        .arg("-v")
        .status()
    {
        Ok(t) => Ok(t),
        Err(_e) => Err("Docker is required".to_string()),
    }
}

pub fn current_working_dir() -> PathBuf {
    current_dir().unwrap()
}
