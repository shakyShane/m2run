use flags::bool::bool_from;
use flags::Flag;

pub fn get_quiet(user_input: &Vec<String>) -> Result<Flag<bool>, String> {
    Ok(Flag {
        value: bool_from(&user_input, &vec!["quiet", "q"]).unwrap_or(false),
        name: "quiet".into(),
        description: "silence the output".into()
    })
}
