use std::io;

mod app;
use app::App;

mod tabs;
use tabs::StatusTab;

mod git;
use git::{get_repository, Git};

mod pages;
use pages::Pages;

mod popup;

use crate::{popup::Popup, tabs::BranchTab};

fn main() -> io::Result<()> {
    let repository = match get_repository() {
        Ok(repo) => repo,
        Err(_e) => {
            eprintln!("❌ Failed to find repository");
            eprintln!("Ether to deep or missing git folder");
            std::process::exit(1);
        }
    };

    let mut terminal = ratatui::init();

    let mut program = App {
        exit: false,
        page: Pages::StatusPAGE,
        status_page: StatusTab::new().into(),
        branch_page: BranchTab::new(),
        git: Git::new(repository),
        message_popup: Popup::new(),
    };

    program.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
