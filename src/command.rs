use std::path::PathBuf;

#[derive(Debug)]
pub struct IncomingCommand {
    pub command: &'static str,
    pub args: Vec<String>,
    pub stdin: &'static str
}
#[derive(Debug)]
pub struct RunContext {
    pub cwd: PathBuf
}
pub fn execute_command() {

}