use std::env::current_dir;

#[derive(Debug)]
pub enum FlagValue<T> {
    Boolean(T),
    String(T),
}

#[derive(Debug)]
pub enum BoolFlag {
    True,
    False,
}

#[derive(Debug)]
pub struct Flag<T> {
    value: FlagValue<T>,
    name: String,
    short: String,
    description: String
}

impl <T> Flag<T> {
    fn get_value(&self) -> &FlagValue<T> {
        match &self.value {
            FlagValue::Boolean(t) => println!("ye"),
            FlagValue::String(t) => println!("ye"),
        }
    }
}

#[derive(Debug)]
pub struct Flags {
    quiet: Flag<BoolFlag>,
    user: Flag<String>,
}

#[test]
fn test_flags() {
    let mut flags = Flags {
        quiet: Flag {
            short: "q".into(),
            name: "quiet".into(),
            description: "Should the output be quiet".into(),
            value: FlagValue::Boolean(BoolFlag::True),
        },
        user: Flag {
            short: "u".into(),
            name: "user".into(),
            description: "The user to run commands under".into(),
            value: FlagValue::String(String::from("www-data")),
        }
    };

    look_at_flags(&mut flags);
}

fn look_at_flags(flags: &mut Flags) {
    flags.user.value = FlagValue::String(String::from("root"));

//    println!("{:?}", flags.quiet.get_value());

    let is_quiet = match flags.quiet.value {
        BoolFlag::True => println!("trupe"),
        BoolFlag::False => println!("trupe"),
    };
}