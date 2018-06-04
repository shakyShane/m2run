use context::current_working_dir;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Options {
    pub cwd: PathBuf,
    pub flags: HashMap<String, String>,
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

fn create_defaults(os_cwd: PathBuf) -> HashMap<String, String> {
    let mut defaults = HashMap::new();

    defaults.insert("cwd".into(), os_cwd.to_string_lossy().into());
    defaults.insert("run_mode".into(), "execute".into());
    defaults.insert("user".into(), "www-data".into());
    defaults
}

pub fn generate_options(raw_args: &Vec<String>, os_cwd: PathBuf) -> Result<Options, String> {

    let defaults = create_defaults(os_cwd);
    let (before, trailing, has_terminator) = split_args(&raw_args);
    let program_args = match has_terminator {
        true => before,
        false => trailing
    };
    let flags = create_flags_hash(opts_without_cmd(&program_args).to_vec(), defaults);
    let cwd_as_buf: PathBuf = flags.get("cwd").unwrap().into();

    Ok(Options {
        cwd: cwd_as_buf,
        flags,
        trailing: trailing.to_vec(),
        raw: raw_args.to_vec(),
    })
}

#[test]
fn test_generate_options_1() {
    let opts = vec!["m2run", "e", "--cwd", "/user", "--run_mode", "dry_run",  "--", "ls"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd).unwrap();
    assert_eq!(opts.flags.get("run_mode"), Some(&"dry_run".to_string()));
    assert_eq!(opts.flags.get("cwd"), Some(&"/user".to_string()));
    assert_eq!(opts.trailing.get(0), Some(&"ls".to_string()));
}
#[test]
fn test_generate_options_2() {
    let opts = vec!["m2run", "e", "ls"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd).unwrap();
    assert_eq!(opts.flags.get("run_mode"), Some(&"execute".to_string()));
    assert_eq!(opts.trailing.get(0), Some(&"ls".to_string()));
}
#[test]
fn test_generate_options_3() {
    let opts = vec!["m2run", "e", "--user", "root", "--", "ls"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd).unwrap();
    assert_eq!(opts.flags.get("run_mode"), Some(&"execute".to_string()));
    assert_eq!(opts.flags.get("user"), Some(&"root".to_string()));
    assert_eq!(opts.trailing.get(0), Some(&"ls".to_string()));
}
#[test]
fn test_generate_options_4() {
    let opts = vec!["m2run", "e", "--user", "--cwd", "/users/kittie", "--", "ls"].iter().map(|x| x.to_string()).collect();
    let os_cwd = PathBuf::from("/users/shane");
    let opts = generate_options(&opts, os_cwd).unwrap();
    assert_eq!(opts.flags.get("cwd"), Some(&"/users/kittie".to_string()));
    assert_eq!(opts.flags.get("user"), Some(&"www-data".to_string()));
    assert_eq!(opts.trailing.get(0), Some(&"ls".to_string()));
}

fn opts_without_cmd(raw_opts: &[String]) -> &[String] {
    match raw_opts.len() {
        0...1 => &[],
        _ => &raw_opts[2..]
    }
}

fn split_args(raw_opts: &Vec<String>) -> (&[String], &[String], bool) {
    let len = raw_opts.len();
    let indexes = 0..len;

    let terminator = indexes
        .zip(raw_opts.iter())
        .find(|&(_i, opt)| *opt == "--");

    match len {
        0...1 => (&[], &[], false),
        _ => match terminator {
            Some((index, _opt)) => {
                (&raw_opts[..index], &raw_opts[(index + 1)..], true)
            },
            None => (&[], &raw_opts[2..], false)
        }
    }
}

fn create_flags_hash(
    opts: Vec<String>,
    defaults: HashMap<String, String>,
) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for (key, val) in defaults.iter() {
        let flag = format!("--{}", key);
        let indexes = 0..opts.len();
        let mut iter = indexes.zip(opts.iter());
        let matches = iter.find(|&(_, opt)| flag == *opt.as_str());

        match matches {
            Some((index, _)) => {
                if let Some(next) = opts.get(index + 1) {
                    let first_char = next.chars().next().unwrap().to_string();
                    match &*first_char {
                        "-" => {
                            map.insert(key.to_string(), val.to_string());
                        },
                        _ => {
                            map.insert(key.to_string(), next.to_string());
                        },
                    }
                } else {
                    println!("key={}", key);
                }
            }
            None => {
                map.insert(key.to_string(), val.to_string());
            }
        }
    }

    map
}

#[test]
fn create_options_hash_test() {
    let mut defaults = HashMap::new();
    defaults.insert("cwd".to_string(), "/Users/shane/Downloads".to_string());
    defaults.insert("name".to_string(), "kittie".to_string());
    let opts = vec![
        "--cwd",
        "/users/shane", // exists in defaults, but is overridden
        "--dc",
        "another", //"doesnt exist in defaults"
    ].iter()
        .map(|x| x.to_string())
        .collect();
    let m = create_flags_hash(opts, defaults);
    assert_eq!(m.get("cwd"), Some(&"/users/shane".to_string()));
    assert_eq!(m.get("name"), Some(&"kittie".to_string()));
    assert_eq!(m.get("dc"), None);
}
