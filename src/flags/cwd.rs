use std::path::PathBuf;
use flags::string::string_from;
use flags::Flag;
use context::RunContextError;

pub fn get_cwd(user_input: &Vec<String>, os_cwd: &PathBuf) -> Result<Flag<PathBuf>, RunContextError> {

    let names = vec!["cwd"];
    let name = String::from("cwd");
    let description = String::from("path to run commands from");

    match string_from(&user_input, &names) {
        /*
         * Did the user provide a cwd flag?
         */
        Some(t) => {
            /*
             * Convert the String path to a PathBuf;
             */
            let pb = PathBuf::from(&t);
            /*
             * Check it exists, if not, this is a fatal error
             */
            if !pb.exists() {
                return Err(RunContextError::CwdNotAvailable(pb.to_owned()));
            }
            /*
             * Here, the user provided a path, and it was verified to exist
             */
            return Ok(Flag {
                value: pb.to_path_buf(),
                name,
                description,
            });
        },
        /*
         * If we get here, it's because
         */
        None => Ok(Flag {
            value: os_cwd.to_path_buf(),
            name,
            description,
        })
    }
}
