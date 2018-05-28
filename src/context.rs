use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RunContext {
    pub cwd: PathBuf,
    pub name: String,
    pub command: String,
    pub opts: HashMap<String, String>,
    pub mode: RunMode,
}

#[derive(Debug)]
pub enum RunMode {
    DryRun,
    Execute,
}

pub fn create_run_context(
    cwd_as_buf: &PathBuf,
    opts: &HashMap<String, String>,
) -> Result<RunContext, String> {
    let context_name = cwd_as_buf.file_name().unwrap();
    let as_string = context_name.to_string_lossy();
    let cmd = env::args().nth(1).or(Some("contrib".to_string())).unwrap();

    let mode: RunMode = match opts.get("run_mode") {
        Some(mode) => match mode.as_str() {
            "execute" | "exe" => RunMode::Execute,
            "dry_run" | "dryrun" | "dryRun" => RunMode::DryRun,
            _ => RunMode::Execute,
        },
        None => RunMode::Execute,
    };

    Ok(RunContext {
        cwd: cwd_as_buf.to_path_buf(),
        name: as_string.to_string(),
        command: cmd,
        opts: opts.clone(),
        mode,
    })
}
