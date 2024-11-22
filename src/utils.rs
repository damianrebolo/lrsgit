use std::env;

#[derive(PartialEq, Debug)]
pub enum Blocks {
    Status,
    Files,
    LocalBranches,
    Commits,
    Stash,
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
