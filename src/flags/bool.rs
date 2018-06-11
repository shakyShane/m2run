pub fn bool_from<'a>(user_input: &Vec<String>, names: &Vec<&'a str>) -> Option<bool> {
    match user_input.iter().find(|x| {
        let short = names.iter().find(|name| format!("-{}", name) == x.as_ref()).map(|_x| true);
        let long = names.iter().find(|name| format!("--{}", name) == x.as_ref()).map(|_x| true);

        return short.unwrap_or(long.unwrap_or(false));
    }) {
        Some(_t) => Some(true),
        None => None
    }
}

#[test]
fn test_bool_from() {
    let user_input: Vec<String> = vec!["--quiet"].iter().map(|x| x.to_string()).collect();
    assert_eq!(bool_from(&user_input, &vec!["quiet"]), Some(true));

//    let user_input: Vec<String> = vec![];
//    assert_eq!(bool_from(&user_input, &vec!["quiet"]), None);
//
//    let user_input: Vec<String> = vec!["-q"].iter().collect();
//    assert_eq!(bool_from(&user_input, &vec!["quiet", "q"]), Some(true));
}
