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

pub fn generate_options(raw_opts: &Vec<String>, os_cwd: PathBuf) -> Result<Options, String> {

    let defaults = create_defaults(os_cwd);
    let trailing = get_trailing(&raw_opts);
    let flags = create_flags_hash(opts_without_cmd(&raw_opts).to_vec(), defaults);
    let cwd_as_buf: PathBuf = flags.get("cwd").unwrap().into();

    Ok(Options {
        cwd: cwd_as_buf,
        flags,
        trailing: trailing.to_vec(),
        raw: raw_opts.to_vec(),
    })
}

fn opts_without_cmd(raw_opts: &Vec<String>) -> &[String] {
    match raw_opts.len() {
        0...1 => &[],
        _ => &raw_opts[2..]
    }
}

fn get_trailing(raw_opts: &Vec<String>) -> &[String] {
    let len = raw_opts.len();
    let indexes = 0..len;

    let terminator = indexes
        .zip(raw_opts.iter())
        .find(|&(_i, opt)| *opt == "--");

    match len {
        0...1 => &[],
        _ => match terminator {
            Some((index, _opt)) => &raw_opts[(index + 1)..],
            None => &raw_opts[2..],
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
        let iter = indexes.zip(opts.iter());

        let matches: Vec<(usize, &String)> =
            iter.filter(|&(_, opt)| flag == *opt.as_str()).collect();

        match matches.get(0) {
            Some(&(index, _)) => {
                if let Some(next) = opts.get(index + 1) {
                    map.insert(key.to_string(), next.to_string());
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
