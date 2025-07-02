use std::io;

mod app;
use app::App;

mod tabs;
use tabs::{StatusBlocks, StatusTab};

mod git;
use git::{get_repository, CommitMode, Git, PushMode};

mod pages;
use pages::Pages;

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
            branch: String::from("master"),
            input: String::new(),
            character_index: 0,
            commit_mode: CommitMode::Normal,
            messages: Vec::new(),
            push_mode: PushMode::Normal,
            push_message: String::from("Are you sure you want to push your work ?"),
        },
    };

    program.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
