use flags::bool::bool_from;
use flags::Flag;
use context::RunContextError;

pub fn get_quiet(user_input: &Vec<String>) -> Result<Flag<bool>, RunContextError> {
    Ok(Flag {
        value: bool_from(&user_input, &vec!["quiet", "q"]).unwrap_or(false),
        name: "quiet".into(),
        description: "silence the output".into()
    })
}
