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

#[derive(Debug, PartialEq)]
pub struct FlagValue<T> {
    pub inner: Flag<T>
}

impl <T> FlagValue<T> {
    pub fn new(x: Flag<T>) -> FlagValue<T> {
        FlagValue { inner: x }
    }
    pub fn value(&self) -> &T {
        &self.inner.value
    }
}

impl <T> fmt::Display for FlagValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.inner.name,
            self.inner.description
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
    pub cwd: FlagValue<PathBuf>,
    pub dry: FlagValue<bool>,
    pub quiet: FlagValue<bool>,
    pub user: FlagValue<String>,
    pub run_mode: FlagValue<RunMode>,
    pub host: FlagValue<String>,
}

impl ProgramFlags {
    fn post_process(&mut self) {
        match *self.dry.value() {
            true => self.run_mode.inner.value = RunMode::DryRun,
            _ => (),
        }
    }
}

pub fn create_program_flags(user_args: &Vec<String>, os_cwd: &PathBuf) -> Result<ProgramFlags, String> {
    let mut p = ProgramFlags {
        cwd: FlagValue::new(get_cwd(&user_args, &os_cwd)?),
        quiet: FlagValue::new(get_quiet(&user_args)?),
        dry: FlagValue::new(get_dry(&user_args)?),
        user: FlagValue::new(get_user(&user_args)?),
        run_mode: FlagValue::new(get_run_mode(&user_args)?),
        host: FlagValue::new(get_host(&user_args)?)
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
    assert_eq!(*flags.run_mode.value(), RunMode::DryRun);
    assert_eq!(*flags.cwd.value(), PathBuf::from("/users/shakyshane"));

    println!("exist? = {}", flags.cwd.value().exists());
}
