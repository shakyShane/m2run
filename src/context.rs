use files::verify_files;
use options;
use options::{Options, get_options};
use std::env;
use std::env::current_dir;
use std::env::set_current_dir;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RunContext {
    pub cwd: PathBuf,
    pub env: HashMap<String, String>,
    pub cwd_file_name: String,
    pub name: String,
    pub user: String,
    pub command: Option<String>,
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
            let cmd = env::args().nth(1);
            is_valid_dir(&options.cwd)
                .and_then(verify_files)
                .and_then(set_working_dir)
                .and_then(|_| create_run_context(options, cmd))
        })
}

pub fn create_run_context(options: options::Options, cmd: Option<String>) -> Result<RunContext, String> {

    let ctx_name    = get_context_name(&options);
    let cmd         = select_cmd(cmd);
    let mode        = select_mode(options.flags.get("run_mode"));
    let user        = select_user(options.flags.get("user"));
    let default_env = get_default_env(&ctx_name);

    Ok(RunContext {
        command: cmd,
        cwd: PathBuf::from(&options.cwd),
        cwd_file_name: ctx_name.to_string(),
        env: default_env,
        options,
        mode,
        name: ctx_name.to_string(),
        user: user.to_string()
    })
}

#[test]
fn test_create_run_context() {
    use options::{generate_options};
    let cwd = "/Users/shakyshane/Downloads/magento2-2.2-develop";
    let raw_opts = vec!["m2run"].iter().map(|x| x.to_string()).collect();
    let opts = generate_options(&raw_opts, PathBuf::from(cwd)).unwrap();
    let ctx = create_run_context(opts, Some("e".into())).unwrap();
    assert_eq!(ctx.env.get("M2RUN_CONTEXT_NAME").unwrap(), "magento2-2.2-develop");
    assert_eq!(ctx.options.flags.get("user").unwrap(), "www-data");
    assert_eq!(ctx.options.flags.get("run_mode").unwrap(), "execute");
}

fn get_context_name(options: &options::Options) -> String {
    let context_name = options.cwd.file_name().unwrap();
    context_name.to_string_lossy().to_string()
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

fn select_cmd(maybe_command: Option<String>) -> Option<String> {
    match maybe_command {
        Some(cmd) => Some(cmd),
        None => Some("help".into())
    }
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
