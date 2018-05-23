use command::RunContext;
use command::IncomingCommand;

pub fn run() -> Result<IncomingCommand, String> {
    let docker_compose_text = include_str!("templates/docker-compose.yml");

    let docker_compose_build_args = vec![
        "-f", "-",
        "up",
    ].iter().map(|x| x.to_string()).collect();

    Ok(IncomingCommand {
        command: "docker-compose",
        args: docker_compose_build_args,
        stdin: docker_compose_text,
    })
}