use std::path::PathBuf;
use std::io::{Error, Write};
use context::RunContext;
use std::fs::File;
use std::fs;
use std::fmt;
use std::ffi::OsStr;

#[derive(Debug, Default)]
pub struct FileWriteOp {
    pub path: PathBuf,
    pub content: String,
    pub description: String,
}

impl FileWriteOp {
    pub fn new(path: PathBuf, content: String) -> FileWriteOp {
        FileWriteOp {
            path,
            content,
            description: "Writes a file to disk".into()
        }
    }
}

#[derive(Debug)]
pub struct FileRemoveOp {
    pub path: PathBuf,
    pub description: String,
}

impl FileRemoveOp {
    pub fn new(path: PathBuf) -> FileRemoveOp {
        FileRemoveOp {
            path,
            description: "Removes a file from disk".into()
        }
    }
}

#[derive(Debug)]
pub enum FileOperationKind {
    Write(FileWriteOp),
    Remove(FileRemoveOp),
}

#[derive(Debug)]
pub struct FileOperation {
    pub kind: FileOperationKind,
}

impl fmt::Display for FileOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            FileOperationKind::Write(ref write_op) => {
                write!(f, "Description: {}\n\n    Dir: {:?}\n    File: {:?}",
                       write_op.description,
                       write_op.path.parent().unwrap_or(&PathBuf::from("n/a")),
                       write_op.path.file_name().unwrap_or(&OsStr::new("n/a")),
                )
            }
            FileOperationKind::Remove(ref remove_op) => {
                write!(f, "Description: {}\n\n    Dir: {:?}\n    File: {:?}",
                       remove_op.description,
                       remove_op.path.parent().unwrap_or(&PathBuf::from("n/a")),
                       remove_op.path.file_name().unwrap_or(&OsStr::new("n/a")),
                )
            }
        }
    }
}

pub fn perform_file_operation(file_operation: &FileOperation, run_context: &RunContext) -> Result<(), Error> {
    match file_operation.kind {
        FileOperationKind::Write(ref file_op) => {
            let joined = run_context.cwd.join(&file_op.path);
            File::create(&joined).and_then(|mut f| f.write_all(file_op.content.as_bytes()))
        }
        FileOperationKind::Remove(ref write_op) => {
            let joined = run_context.cwd.join(&write_op.path);
            fs::remove_file(joined)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use context::create_run_context;
    use std::env::current_dir;

    #[test]
    fn test_write () {
        let op = FileWriteOp::new(PathBuf::from(".dockerignore"), String::from(".git\nvendor"));
        let t = FileOperation {
            kind: FileOperationKind::Write(op)
        };
        use options::{generate_options};
        let raw_opts = vec!["m2run"].iter().map(|x| x.to_string()).collect();
        let test_dir = current_dir().unwrap().join("fixtures");
        println!("{:?}", test_dir);
        let opts = generate_options(&raw_opts, &test_dir).unwrap();
        let run_context = create_run_context(opts, Some("e".into())).unwrap();
        perform_file_operation(&t, &run_context);
    }
}
