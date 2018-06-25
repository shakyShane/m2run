use command::ExecCommand;
use file_operation::FileOperation;

#[derive(Debug)]
pub enum Task<'a> {
    ExecCommand(ExecCommand<'a>),
    FileOperation(FileOperation)
}