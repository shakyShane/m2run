use std::process::Stdio;
use std::process::Command;
use std::io::Write;
use std::path::PathBuf;
use std::env::set_current_dir;
use std::ffi::OsStr;

const TAG_PREFIX: &'static str = "m2run";
const TAG_SUFFIX: &'static str = "php";

pub fn build_dockerfile(cwd: &PathBuf) {

    match set_current_dir(cwd) {
        Ok(_) => {
            println!("cwd = {:?}", cwd);
            docker_build_command(cwd);
        },
        Err(e) => {
            println!("could not set dir, {:?}", e)
        }
    }
}

fn docker_build_command(cwd: &PathBuf) {

    let cwd_base_name = cwd.file_name().expect("Could not determine base_name of directory");

    let docker_build_image_text = include_str!("templates/Dockerfile");
    let docker_build_tag: String = create_build_tag(&cwd_base_name);
    println!("docker_build_tag = {}", docker_build_tag);

    let mut process = match Command::new("docker")
        .args(&[
            "build",
            "-f", "-",
            "-t", &*docker_build_tag,
            "."])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn() {
        Err(why) => panic!("couldn't spawn docker: {}", why),
        Ok(process) => process,
    };

    process.stdin.as_mut().unwrap().write_all(docker_build_image_text.as_bytes());
    let output = process.wait_with_output().unwrap();
}

fn create_build_tag(base_name: &OsStr) -> String {
    println!("base_name = {:?}", base_name);
    format!("{}__{}__{}", TAG_PREFIX, base_name.to_string_lossy(), TAG_SUFFIX)
}
