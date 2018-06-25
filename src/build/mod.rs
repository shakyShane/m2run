use context::RunMode;

pub mod caddy;
pub mod php;

const TAG_PREFIX: &'static str = "m2run";
pub const PHP_TAG_SUFFIX: &'static str = "php";
pub const CADDY_TAG_SUFFIX: &'static str = "caddy";

fn create_build_arg(
    name: &str,
    text: &str,
    origin: &str,
    mode: &RunMode,
) -> String {
    match mode {
        &RunMode::Execute => format!("{}={}", name, text),
        &RunMode::DryRun => format!("{}={}", name, origin),
    }
}

pub fn create_build_tag(base_name: &str, suffix: &str) -> String {
    //    println!("base_name = {:?}", base_name);
    format!("{}__{}__{}", TAG_PREFIX, base_name, suffix)
}
