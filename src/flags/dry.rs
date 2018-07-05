use flags::bool::bool_from;
use flags::Flag;
use context::RunContextError;

pub fn get_dry(user_input: &Vec<String>) -> Result<Flag<bool>, RunContextError> {
    Ok(Flag::new(
        bool_from(&user_input, &vec!["dry"]).unwrap_or(false),
        "dry",
        "Shortcut for setting the run mode"
    ))
}
