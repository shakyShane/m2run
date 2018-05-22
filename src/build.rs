use std::process::Stdio;
use std::process::Command;
use std::io::Write;

pub fn build_dockerfile() {
    let build_text = "FROM bluestreak/entry";

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

