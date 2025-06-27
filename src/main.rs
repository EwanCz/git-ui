use git2::Repository;
use std::io;

mod app;
use app::App;

mod tabs;
use tabs::{StatusBlocks, StatusTab};

mod git;
use git::{CommitMode, Git};

mod pages;
use pages::Pages;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let repository = Repository::open(".")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Git error: {}", e)))?;

    let mut program = App {
        exit: false,
        page: Pages::StatusPAGE,
        status_page: StatusTab {
            line_in_file: 0,
            line_in_folder_unstaged: 0,
            line_in_folder_staged: 0,
            focused_block: StatusBlocks::Unstaged,
            nb_unstaged_file: 0,
            nb_staged_file: 0,
            filepath_diff: String::new(),
        }
        .into(),
        git: Git {
            repo: repository,
            input: String::new(),
            character_index: 0,
            commit_mode: CommitMode::Normal,
            messages: Vec::new(),
        },
    };

    program.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
