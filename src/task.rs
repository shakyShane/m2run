use command::ExecCommand;

#[derive(Debug)]
pub enum Task<'a> {
    ExecCommand(ExecCommand<'a>)
}