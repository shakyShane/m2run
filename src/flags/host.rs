use flags::Flag;
use flags::string::string_from;

pub fn get_host(user_input: &Vec<String>) -> Result<Flag<String>, String> {
    let value = string_from(&user_input, &vec!["host"])
        .unwrap_or("contrib.m2".into());

    Ok(Flag {
        value,
        name: "host".into(),
        description: "the domain name that Magento will run under".into()
    })
}