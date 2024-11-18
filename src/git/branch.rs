use git2::{BranchType, Repository};

const REPO_PATH: &str = ".";

pub fn get_current_branch() -> String {
    let repo = Repository::open(REPO_PATH).unwrap();
    let head = repo.head().unwrap();
    let head = head.shorthand().unwrap();
    head.to_string()
}

pub fn get_local_branches() -> Vec<String> {
    let repo = Repository::open(REPO_PATH).unwrap();
    repo.branches(Some(BranchType::Local))
        .unwrap()
        .map(|branch| {
            let (branch, _) = branch.unwrap();
            branch.name().unwrap().unwrap().to_string()
        })
        .collect()
}

pub fn new_branch(name: String, base: String) {}
