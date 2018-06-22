use flags::string::string_from;
use flags::Flag;
use context::RunMode;
use context::RunContextError;

pub fn get_run_mode(user_input: &Vec<String>) -> Result<Flag<RunMode>, RunContextError> {

    let keys = &vec!["run_mode", "runmode", "run-mode", "runMode"];
    let default = "execute";
    let value = match string_from(&user_input, keys).unwrap_or(default.into()).as_ref() {
        "dryrun" | "dry-run" | "dry_run" | "dryRun" => RunMode::DryRun,
        _ => RunMode::Execute,
    };

    Ok(Flag {
        value,
        name: "run_mode".into(),
        description: "the run mode, can be either execute or dry-run".into(),
    })
}

