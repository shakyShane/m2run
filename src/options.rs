use command::current_working_dir;
use files::verify_files;
use std::collections::HashMap;
use std::env;
use std::env::set_current_dir;
use std::path::PathBuf;

pub fn get_options_hash() -> Result<(PathBuf, HashMap<String, String>), String> {
    collect_args().and_then(generate_options_hash)
}

fn collect_args() -> Result<Vec<String>, String> {
    let raw_opts: Vec<String> = env::args().collect();
    let arg_len = raw_opts.len();
    match arg_len {
        1 => Err("No command provided".to_string()),
        _num => Ok(raw_opts),
    }
}

fn generate_options_hash(
    raw_opts: Vec<String>,
) -> Result<(PathBuf, HashMap<String, String>), String> {
    let os_cwd = current_working_dir();
    let mut defaults = HashMap::new();
    defaults.insert("cwd".to_string(), os_cwd.to_string_lossy().to_string());
    defaults.insert("run_mode".to_string(), "execute".into());
    let parsed_options = create_options_hash(raw_opts[2..].to_vec(), defaults);
    let cwd_as_buf: PathBuf = parsed_options.get("cwd").unwrap().into();

    is_valid_dir(&cwd_as_buf)
        .and_then(verify_files)
        .and_then(set_working_dir)
        .and_then(|_x| Ok((cwd_as_buf, parsed_options.clone())))
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

fn create_options_hash(
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
    let m = create_options_hash(opts, defaults);
    assert_eq!(m.get("cwd"), Some(&"/users/shane".to_string()));
    assert_eq!(m.get("name"), Some(&"kittie".to_string()));
    assert_eq!(m.get("dc"), None);
}
