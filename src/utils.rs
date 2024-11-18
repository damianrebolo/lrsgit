use std::env;

#[derive(PartialEq)]
pub enum Blocks {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Main,
    Logs,
}

pub fn get_current_folder() -> String {
    env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
