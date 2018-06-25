use std::path::PathBuf;
use std::io::{Error, ErrorKind, Write};
use context::RunContext;

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
    unimplemented!();
}
