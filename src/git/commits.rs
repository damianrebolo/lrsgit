use std::process::Command;

#[derive(Debug)]
pub struct CommitInfo {
    pub short_hash: String,
    pub user_initials: String,
    pub upstreamed: bool,
    pub message: String,
}

pub fn get_commits(branch_name: &str) -> Option<Vec<CommitInfo>> {
    let format = "%h %an %s";
    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=format:".to_string() + format)
        .arg(branch_name)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                // Parse each line of the output into a CommitInfo struct
                let commits = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|line| {
                        let parts: Vec<&str> = line.splitn(4, ' ').collect();
                        if parts.len() < 4 {
                            return None; // Skip malformed lines
                        }

                        let short_hash = parts[0].to_string();
                        let user_initials = format!("{} {}", parts[1], parts[2])
                            .as_str()
                            .split_whitespace()
                            .map(|name| name.chars().next().unwrap_or('_').to_ascii_uppercase())
                            .collect();
                        let message = parts[3].to_string();
                        let upstreamed = message.to_lowercase().contains("upstream");

                        Some(CommitInfo {
                            short_hash,
                            user_initials,
                            upstreamed,
                            message,
                        })
                    })
                    .filter_map(|x| x)
                    .collect();
                Some(commits)
            } else {
                // Capture error from stderr if git log fails
                None
            }
        }
        Err(err) => None,
    }
}
