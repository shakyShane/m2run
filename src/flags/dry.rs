use flags::bool::bool_from;
use flags::Flag;

pub fn get_dry(user_input: &Vec<String>) -> Result<Flag<bool>, String> {
    Ok(Flag {
        value: bool_from(&user_input, &vec!["dry"]).unwrap_or(false),
        name: "dry".into(),
        description: "Shortcut for setting the run mode".into()
    })
}
