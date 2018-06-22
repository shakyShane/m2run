use std::path::{Path, PathBuf};
use context::RunContextError;

#[derive(Debug, Clone, PartialEq)]
pub struct FileLookup {
    pub path: String,
    pub exists: bool,
    pub absolute: PathBuf,
}

pub fn verify_files(cwd: &PathBuf) -> Result<&PathBuf, RunContextError> {
    let required_files = vec!["composer.json", "composer.lock"];
    let file_statues = required_files_status(&required_files, &cwd);
    let (_found, missing): (Vec<&FileLookup>, Vec<&FileLookup>) =
        file_statues.iter().partition(|x| x.exists);

    match missing.len() {
        0 => Ok(&cwd),
        _num => {
            let mut new_vec: Vec<FileLookup> = Vec::new();
            for item in missing {
                new_vec.push(item.clone());
            }
            Err(RunContextError::MissingFiles(new_vec, cwd.to_owned()))
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
