use std::path::PathBuf;
use flags::run_mode::get_run_mode;
use flags::user::get_user;
use flags::dry::get_dry;
use flags::quiet::get_quiet;
use flags::cwd::get_cwd;
use std::fmt;
use context::RunMode;
use flags::host::get_host;

mod run_mode;
mod user;
mod dry;
mod quiet;
mod cwd;
mod string;
mod bool;
mod host;

impl <T> fmt::Display for Flag<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name,
            self.description
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Flag<T> {
    pub value: T,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub struct ProgramFlags {
    pub cwd: Flag<PathBuf>,
    pub dry: Flag<bool>,
    pub quiet: Flag<bool>,
    pub user: Flag<String>,
    pub run_mode: Flag<RunMode>,
    pub host: Flag<String>,
}

impl ProgramFlags {
    fn post_process(&mut self) {
        match self.dry.value {
            true => self.run_mode.value = RunMode::DryRun,
            _ => (),
        }
    }
}

pub fn create_program_flags(user_args: &Vec<String>, os_cwd: &PathBuf) -> Result<ProgramFlags, String> {
    let mut p = ProgramFlags {
        cwd: get_cwd(&user_args, &os_cwd)?,
        quiet: get_quiet(&user_args)?,
        dry: get_dry(&user_args)?,
        user: get_user(&user_args)?,
        run_mode: get_run_mode(&user_args)?,
        host: get_host(&user_args)?
    };

    p.post_process();

    Ok(p)
}

#[test]
fn test_post_process() {
    use std::env::current_dir;
    let os_cwd = current_dir().unwrap();
    let args = &vec!["--dry", "--cwd", "/users/shakyshane"].iter().map(|x| x.to_string()).collect();
    let flags = create_program_flags(args, &os_cwd).unwrap();
    assert_eq!(flags.run_mode.value, RunMode::DryRun);
    assert_eq!(*flags.cwd.value, PathBuf::from("/users/shakyshane"));

    println!("exist? = {}", flags.cwd.value.exists());
}
