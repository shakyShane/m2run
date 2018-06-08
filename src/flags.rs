use std::env::current_dir;
use std::fmt;

#[derive(Debug)]
pub enum Boolean {
    True,
    False
}

#[derive(Debug)]
pub struct BooleanFlag {
    value: Boolean,
    name: String,
    short: String,
    description: String
}


#[derive(Debug)]
pub struct StringFlag {
    value: String,
    name: String,
    short: String,
    description: String
}

#[derive(Debug)]
enum RunMode {
    Execute,
    DryRun
}

#[derive(Debug)]
pub struct ChoiceFlag<T> {
    value: T,
    name: String,
    short: String,
    description: String
}


#[derive(Debug)]
pub struct Flags {
    quiet: BooleanFlag,
    user: StringFlag,
    run_mode: ChoiceFlag<RunMode>,
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hello world!")
    }
}

#[test]
fn test_flags() {
    let mut flags = Flags {
        quiet: BooleanFlag {
            short: "q".into(),
            name: "quiet".into(),
            description: "Should the output be quiet".into(),
            value: Boolean::True,
        },
        user: StringFlag {
            short: "u".into(),
            name: "user".into(),
            description: "The user to run commands under".into(),
            value: String::from("www-data"),
        },
        run_mode: ChoiceFlag {
            short: "u".into(),
            name: "user".into(),
            description: "The user to run commands under".into(),
            value: RunMode::Execute,
        },
    };

    look_at_flags(&mut flags);
}

fn look_at_flags(flags: &mut Flags) {

    println!("{}", flags);
//    match flags.run_mode.value {
//        RunMode::Execute => println!("Execute the command bro"),
//        RunMode::DryRun => println!("It's a dry run, don't panic"),
//    }
//
//    match flags.quiet.value {
//        Boolean::True => println!("quiet == true"),
//        Boolean::False => println!("quiet == false"),
//    }

//    assert_eq!(flags.quiet.get_value(), true);
//    assert_eq!(flags.user.get_value(), "www-data");
}
