use std::collections::HashMap;

#[derive(Debug)]
pub enum FlagKind {
    Boolean,
    String,
//    Array
}

#[derive(Debug)]
pub struct Flag<'a> {
    kind: FlagKind,
    name: &'a str,
    short: &'a str,
    description: &'a str,
}

#[test]
fn test_flags() {
    let mut f: HashMap<&str, Flag> = HashMap::new();
    let items = vec![
        Flag {
            short: "q",
            name: "quiet",
            description: "Should the output be quiet",
            kind: FlagKind::Boolean,
        },
        Flag {
            short: "u",
            name: "user",
            description: "The user to run commands under",
            kind: FlagKind::String,
        }
    ];

    for i in items {
        f.insert(i.name, i);
    }

    let input = "q";

    let _matched = f.iter().find(|&(_key, flag)| {
        flag.short == input
    });
}