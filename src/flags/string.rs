pub fn string_from<'a>(user_input: &Vec<String>, names: &Vec<&'a str>) -> Option<String> {
    match user_input.iter().enumerate()
        .find(|&(_index, x)| {
            match names.iter().find(|n| format!("--{}", n) == x.as_ref()) {
                Some(_t) => true,
                None => false
            }
        })
        .map(|(index, _x)| user_input.get(index + 1))
        {
            Some(t) => {
                match t {
                    Some(value) => Some(value.to_string()),
                    None => None
                }
            },
            None => None
        }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_args(args: Vec<&str>) -> Vec<String> {
        args.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_string_from() {

        let user_input = test_args(vec!["--user", "shane"]);
        assert_eq!(string_from(&user_input, &vec!["user"]), Some("shane".to_string()));

        let user_input = test_args(vec!["--user"]);
        assert_eq!(string_from(&user_input, &vec!["user"]), None);

        let user_input = vec![];
        assert_eq!(string_from(&user_input, &vec!["user"]), None);

        let user_input = test_args(vec!["--runmode", "dry"]);
        assert_eq!(string_from(&user_input, &vec!["run_mode", "runmode"]), Some("dry".to_string()));
    }
}

