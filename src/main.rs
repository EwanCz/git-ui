use std::io;

mod app;
use app::App;

mod status;
use status::Status;
use status::StatusBlocks;

mod pages;
use pages::Pages;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut program = App {
        exit: false,
        page: Pages::StatusPAGE,
        git: git_info::get(),
        status_page: Status {
            line_in_file: 0,
            line_in_folder_unstaged: 0,
            line_in_folder_staged: 0,
            focused_block: StatusBlocks::Diff,
        },
    };

    program.run(&mut terminal)?;
    ratatui::restore();
    let info = git_info::get();
    println!(
        "User Name: {}",
        info.user_name.unwrap_or("Unknown".to_string())
    );
    println!(
        "User Email: {}",
        info.user_email.unwrap_or("Unknown".to_string())
    );
    println!("Dirty: {}", info.dirty.unwrap_or(false));
    println!(
        "Current Branch: {}",
        info.current_branch.unwrap_or("Unknown".to_string())
    );
    println!(
        "Last Commit Hash: {}",
        info.head.last_commit_hash.unwrap_or("Unknown".to_string())
    );
    println!(
        "Last Commit Hash (short): {}",
        info.head
            .last_commit_hash_short
            .unwrap_or("Unknown".to_string())
    );
    println!("Config: {:#?}", info.config.unwrap());
    println!("Branches: {:#?}", info.branches.unwrap_or(vec![]));
    Ok(())
}
