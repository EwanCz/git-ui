//use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Paragraph},
    Frame,
};
use std::fs;

#[derive(PartialEq, Eq)]
pub enum StatusBlocks {
    Unstaged,
    Staged,
    Diff,
}

pub struct Status {
    pub line_in_file: u16,
    pub line_in_folder_unstaged: u16,
    pub line_in_folder_staged: u16,
    pub focused_block: StatusBlocks,
}

impl Status {
    pub fn draw(&self, frame: &mut Frame, content: Rect) {
        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(content);
        let [top_left, bottom_left] = Layout::vertical([Constraint::Fill(1); 2]).areas(left);
        let content = fs::read_to_string("Cargo.lock")
            .unwrap_or_else(|_| "Error: Could not read file".to_string());

        let diff = Paragraph::new(content)
            .style(Style::default())
            .block(Block::bordered().title("Diff"))
            .scroll((self.line_in_file, 0));

        frame.render_widget(diff, right);
        frame.render_widget(Block::bordered().title("unstaged"), top_left);
        frame.render_widget(Block::bordered().title("stagged"), bottom_left);
    }
}
