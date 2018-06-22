use flags::string::string_from;
use flags::Flag;
use context::RunContextError;

pub fn get_user(user_input: &Vec<String>) -> Result<Flag<String>, RunContextError> {
    Ok(Flag {
        value: string_from(&user_input, &vec!["user"]).unwrap_or("www-data".into()),
        name: "user".into(),
        description: "the user under which to run commands".into()
    })
}
