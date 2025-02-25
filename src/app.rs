use crate::git::branch::{get_current_branch, get_local_branches};
use crate::git::commits::{get_commits, CommitInfo};
use crate::utils::{get_current_folder, Blocks};

use crossterm::event::KeyCode;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub show_new_branch_popup: bool,
    pub current_folder: String,
    pub current_block: Blocks,
    pub branches: Vec<String>,
    pub current_branch: String,
    pub commits: Vec<CommitInfo>,
}

impl App {
    pub fn new() -> Self {
        let current_branch = get_current_branch().unwrap();
        let commits = get_commits(current_branch.as_str()).unwrap();
        App {
            should_quit: false,
            show_new_branch_popup: false,
            current_block: Blocks::Status,
            current_folder: get_current_folder(),
            branches: get_local_branches().unwrap(),
            current_branch,
            commits,
        }
    }

    // update widgets
    pub fn on_tick(&mut self) {
        // self.branches = get_local_branches();
        // self.current_branch = get_current_branch();
    }

    // handle key events
    pub fn on_key(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('1') => self.current_block = Blocks::Status,
            KeyCode::Char('2') => self.current_block = Blocks::Files,
            KeyCode::Char('3') => self.current_block = Blocks::LocalBranches,
            KeyCode::Char('4') => self.current_block = Blocks::Commits,
            KeyCode::Char('5') => self.current_block = Blocks::Stash,
            KeyCode::Char('n') => {
                if self.current_block == Blocks::LocalBranches {
                    self.show_new_branch_popup = true;
                }
            }
            _ => {}
        }
    }
}
