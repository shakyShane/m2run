use std::path::PathBuf;
use std::io::{Error, Write};
use context::RunContext;
use context::create_run_context;
use std::fs::File;
use std::env::current_dir;
use std::fs;

#[derive(Debug)]
pub struct FileWriteOp {
    pub path: PathBuf,
    pub content: String,
}

#[derive(Debug)]
pub struct FileRemoveOp {
    pub path: PathBuf,
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

pub fn perform_file_operation(file_operation: &FileOperation, run_context: &RunContext) -> Result<(), Error> {
    match file_operation.kind {
        FileOperationKind::Write(ref write_op) => {
            let joined = run_context.cwd.join(&write_op.path);
            File::create(&joined).and_then(|mut f| f.write_all(write_op.content.as_bytes()))
        }
        FileOperationKind::Remove(ref write_op) => {
            let joined = run_context.cwd.join(&write_op.path);
            fs::remove_file(joined)
        }
    }
}

#[test]
fn test_write () {
    let op = FileWriteOp {
        path: PathBuf::from(".dockerignore"),
        content: String::from(".git\nvendor"),
    };
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
