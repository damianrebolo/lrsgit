use git2::{BranchType, Repository};

pub fn get_localbranches() -> Vec<String> {
    let repo = Repository::open(".").unwrap();
    repo.branches(Some(BranchType::Local))
        .unwrap()
        .map(|branch| {
            let (branch, _) = branch.unwrap();
            branch.name().unwrap().unwrap().to_string()
        })
        .collect()
}

pub fn new_branch(name: String, base: String) {}
