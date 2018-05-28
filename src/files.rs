use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileLookup {
    pub path: String,
    pub exists: bool,
    pub absolute: PathBuf,
}

pub fn verify_files(cwd: &PathBuf) -> Result<&PathBuf, String> {
    let required_files = vec!["composer.json", "composer.lock"];
    let file_statues = required_files_status(&required_files, &cwd);
    let (_found, missing): (Vec<&FileLookup>, Vec<&FileLookup>) =
        file_statues.iter().partition(|x| x.exists);

    match missing.len() {
        0 => Ok(&cwd),
        _num => {
            println!(
                "Cannot continue since the following {} file(s) are missing:",
                _num
            );
            missing.iter().for_each(|x| println!("---> {}", x.path));
            println!("cwd: {:?}", cwd);
            Err("Could not verify files".to_string())
        }
    }
}

pub fn required_files_status(files: &Vec<&str>, cwd: &PathBuf) -> Vec<FileLookup> {
    return files
        .into_iter()
        .map(|relative| (relative, Path::join(cwd, relative)))
        .map(|(relative, absolute)| FileLookup {
            path: relative.to_string(),
            exists: absolute.exists(),
            absolute,
        })
        .collect();
}
