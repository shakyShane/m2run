use context::current_working_dir;
use std::env;
use std::path::PathBuf;
use flags::create_program_flags;
use flags::ProgramFlags;

#[derive(Debug, PartialEq)]
pub struct Options {
    pub flags: ProgramFlags,
    pub trailing: Vec<String>,
    pub raw: Vec<String>,
}

pub fn get_options() -> Result<Options, String> {
    collect_args().and_then(|args| generate_options(&args, current_working_dir()))
}

fn collect_args() -> Result<Vec<String>, String> {
    let raw_opts: Vec<String> = env::args().collect();
    let arg_len = raw_opts.len();
    match arg_len {
        1 => Err("No command provided".into()),
        _num => Ok(raw_opts),
    }
}


pub fn generate_options(raw_args: &Vec<String>, os_cwd: PathBuf) -> Result<Options, String> {

    let (before, trailing, has_terminator) = split_args(&raw_args);

    let program_args = match has_terminator {
        true => before,
        false => trailing
    };

    create_program_flags(&program_args.to_vec(), &os_cwd)
        .and_then(|flags| {
            Ok(Options {
                flags,
                trailing: trailing.to_vec(),
                raw: raw_args.to_vec(),
            })
        })
}

#[test]
fn test_generate_options_1() {
    let opts = vec!["m2run", "e", "--cwd", "/user", "--run_mode", "dry_run",  "--", "ls"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd);

    match opts {
        Ok(_r) => {},
        Err(string) => assert_eq!(string, "Could not use \"/user\" as cwd, path did not exist")
    }
}
#[test]
fn test_generate_options_2() {
    let opts = vec!["m2run", "e", "ls"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd);

    match opts {
        Ok(_r) => {
            assert_eq!(_r.trailing.len(), 1);
            assert_eq!(_r.trailing.get(0).unwrap(), "ls");
        },
        _ => {}
    }
}
#[test]
fn test_generate_options_3() {
    use context::RunMode;
    let opts = vec!["m2run", "e", "--user", "root", "--", "ls"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd).unwrap();
    assert_eq!(opts.flags.run_mode.value, RunMode::Execute);
    assert_eq!(opts.flags.user.value, "root");
    assert_eq!(opts.trailing.get(0), Some(&"ls".to_string()));
}
#[test]
fn test_generate_options_4() {
    let opts = vec!["m2run", "e", "--user", "--cwd", "/users/kittie", "--", "ls"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd);

    match opts {
        Ok(_) => {},
        Err(e) => {
            assert_eq!(e, "Could not use \"/users/kittie\" as cwd, path did not exist")
        }
    }
}
#[test]
fn test_generate_options_no_host() {
    let opts = vec!["m2run", "c"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd).unwrap();

    assert_eq!(opts.flags.host.value, "contrib.m2");
}
#[test]
fn test_generate_options_with_host() {
    let opts = vec!["m2run", "c", "--host", "test.m2"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd).unwrap();

    assert_eq!(opts.flags.host.value, "test.m2");
}

fn split_args(raw_opts: &Vec<String>) -> (&[String], &[String], bool) {
    let len = raw_opts.len();

    let terminator = raw_opts.iter()
        .position(|opt| opt == "--");

    match len {
        0...1 => (&[], &[], false),
        _ => match terminator {
            Some(index) => {
                (&raw_opts[..index], &raw_opts[(index + 1)..], true)
            },
            None => (&[], &raw_opts[2..], false)
        }
    }
}
#[test]
fn test_split_args() {
    let opts = vec!["one", "--", "two"].iter().map(|x| x.to_string()).collect();
    let (_before, trailing, has_terminator) = split_args(&opts);
    assert_eq!(trailing.len(), 1);
    assert_eq!(trailing.get(0).unwrap(), "two");
    assert_eq!(has_terminator, true);
}
#[test]
fn test_split_args_2() {
    let opts = vec!["one", "two"].iter().map(|x| x.to_string()).collect();
    let (_before, trailing, has_terminator) = split_args(&opts);
    assert_eq!(trailing.len(), 0);
    assert_eq!(has_terminator, false);
}
#[test]
fn test_split_args_3() {
    let opts = vec!["one", "--", "two", "--", "three"].iter().map(|x| x.to_string()).collect();
    let (_before, trailing, has_terminator) = split_args(&opts);
    assert_eq!(trailing.len(), 3);
    assert_eq!(trailing.get(0).unwrap(), "two");
    assert_eq!(trailing.get(1).unwrap(), "--");
    assert_eq!(trailing.get(2).unwrap(), "three");
    assert_eq!(has_terminator, true);
}
