use std::process::Stdio;
use std::process::Command;
use std::io::Write;
use std::path::PathBuf;
use std::env::set_current_dir;

pub fn build_dockerfile(cwd: &PathBuf) {

    match set_current_dir(cwd) {
        Ok(_) => {
            println!("cwd = {:?}", cwd);
            docker_build_command();
        },
        Err(e) => {
            println!("could not set dir, {:?}", e)
        }
    }
}

fn docker_build_command() {
    let build_text = include_str!("templates/Dockerfile");

    // Spawn the `docker build` command
    let mut process = match Command::new("docker")
        .args(&["build", "-f", "-", "."])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn() {
            Err(why) => panic!("couldn't spawn docker: {}", why),
            Ok(process) => process,
        };

    process.stdin.as_mut().unwrap().write_all(build_text.as_bytes());
    let output = process.wait_with_output().unwrap();
}

