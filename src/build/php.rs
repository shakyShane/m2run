use build::PHP_TAG_SUFFIX;
use build::create_build_tag;
use command::ExecCommand;
use context::RunContext;
use std::collections::HashMap;
use file_operation::{FileOperation, FileOperationKind, FileWriteOp};
use file_operation::FileRemoveOp;

pub fn docker_build_php_command(run_context: &RunContext) -> ExecCommand {
    let docker_build_image_text = include_str!("../templates/contrib/Dockerfile");
    let docker_build_tag: String = create_build_tag(&run_context.cwd_file_name, PHP_TAG_SUFFIX);

    let docker_build_args = vec![
        "build",
        "-f",
        "-",
        "-t",
        &docker_build_tag,
        "."
    ].iter()
        .map(|x| x.to_string())
        .collect();

    ExecCommand {
        command: "docker",
        args: docker_build_args,
        stdin: docker_build_image_text,
        env: HashMap::new(),
        desc: "Builds the PHP image",
    }
}

pub fn docker_ignore_write(run_context: &RunContext) -> FileOperation {
    let op = FileWriteOp::new(
        run_context.cwd.join(".dockerignore"),
        String::from(include_str!("../templates/contrib/.dockerignore"))
    );
    FileOperation {
        kind: FileOperationKind::Write(op)
    }
}

pub fn docker_ignore_remove(run_context: &RunContext) -> FileOperation {
    let op = FileRemoveOp::new(run_context.cwd.join(".dockerignore"));
    FileOperation {
        kind: FileOperationKind::Remove(op)
    }
}