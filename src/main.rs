use git2::{Error as GitError, Repository};
use std::io;

mod app;
use app::App;

mod tabs;
use tabs::StatusTab;

mod git;
use git::{get_repository, Git};

mod pages;
use pages::Pages;

use crate::tabs::BranchTab;

fn main() -> io::Result<()> {
    let repository = match get_repository() {
        Ok(repo) => repo,
        Err(_e) => {
            eprintln!("❌ Failed to find repository");
            eprintln!("Ether to deep or missing git folder");
            std::process::exit(1);
        }
    };
    let current_branch = match get_current_branch_name(&repository) {
        Ok(branch_name) => branch_name,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut terminal = ratatui::init();

    let mut program = App {
        exit: false,
        page: Pages::StatusPAGE,
        status_page: StatusTab::new().into(),
        branch_page: BranchTab::new(),
        git: Git::new(repository, current_branch),
    };

    program.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}

fn get_current_branch_name(repo: &Repository) -> Result<String, GitError> {
    let head = repo.head()?;

    if let Some(name) = head.shorthand() {
        Ok(name.to_string())
    } else {
        Err(GitError::from_str("Unable to get branch name"))
    }
}
