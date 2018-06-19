use flags::Flag;
use flags::string::string_from;

pub fn get_host(user_input: &Vec<String>) -> Result<Flag<String>, String> {
    Ok(Flag {
        value: string_from(&user_input, &vec!["host"]).unwrap_or("contrib.m2".into()),
        name: "host".into(),
        description: "the domain name that Magento will run under".into(),
    })
}