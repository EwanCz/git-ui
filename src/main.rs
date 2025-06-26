use std::io;

mod app;
use app::App;

mod tabs;
use tabs::{StatusBlocks, StatusTab};

mod git;

mod pages;
use pages::Pages;

fn main() -> io::Result<()> {
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
    };

    program.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
