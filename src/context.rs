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

#[derive(Debug)]
pub struct RunContext {
    pub cwd: PathBuf,
    pub cwd_file_name: String,
    pub name: String,
    pub command: String,
    pub options: options::Options,
    pub mode: RunMode,
}

#[derive(Debug)]
pub enum RunMode {
    DryRun,
    Execute,
}

pub fn create_run_context(options: options::Options) -> Result<RunContext, String> {
    let cwd_as_buf = options.cwd.to_path_buf();
    let context_name = cwd_as_buf.file_name().unwrap();
    let as_string = context_name.to_string_lossy();
    let cmd = env::args().nth(1).or(Some("contrib".to_string())).unwrap();

    let mode: RunMode = match options.flags.get("run_mode") {
        Some(mode) => match mode.as_str() {
            "execute" | "exe" => RunMode::Execute,
            "dry_run" | "dryrun" | "dryRun" => RunMode::DryRun,
            _ => RunMode::Execute,
        },
        None => RunMode::Execute,
    };

    Ok(RunContext {
        cwd: options.cwd.to_path_buf(),
        name: as_string.to_string(),
        command: cmd,
        cwd_file_name: options
            .cwd
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        options: options,
        mode,
    })
}

pub fn get_run_context() -> Result<RunContext, String> {
    has_docker()
        .and_then(|_x| get_options())
        .and_then(|options: Options| {
            is_valid_dir(&options.cwd)
                .and_then(verify_files)
                .and_then(set_working_dir)
                .and_then(|_| create_run_context(options))
        })
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
