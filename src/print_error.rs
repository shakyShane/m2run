use context::RunContextError;

pub fn print_error(err: RunContextError) {
    match err {
        RunContextError::CwdNotAvailable(path_buf) => {
            println!("{:?} did not exist", path_buf);
        }
        RunContextError::MissingFiles(missing_files, cwd) => {
            println!(
                "{} file(s) are missing in {:?}",
                missing_files.len(),
                cwd
            );
            missing_files.iter().for_each(|x| println!("---> {}", x.path));
        }
        RunContextError::MissingCommand => {
            println!("You didn't specify a command.\n");
            println!("Examples of valid commands:");
            println!("    m2run start");
            println!("    m2run stop");
            println!("    m2run exec ls");
        }
        RunContextError::RunContextGeneric(str) => {
            println!("{}", str);
        }
    }
}