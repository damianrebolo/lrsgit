use std::process::Command;

pub fn get_current_branch() -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn get_local_branches() -> Option<Vec<String>> {
    let output = Command::new("git").arg("branch").output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let branches = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|line| line.trim().trim_start_matches('*').trim().to_string())
                    .collect();
                Some(branches)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn create_branch(branch_name: &str) -> Result<(), String> {
    let output = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(()) // Successfully switched branches
            } else {
                // Capture the error message from stderr
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        }
        Err(err) => Err(format!("Failed to execute git command: {}", err)),
    }
}

pub fn delete_branch(branch_name: &str) -> Result<(), String> {
    Ok(())
}
