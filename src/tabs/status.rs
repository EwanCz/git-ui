use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};
use std::fs;

use crate::git::{get_files, GitFile, TypeStaged};

#[derive(PartialEq, Eq)]
pub enum StatusBlocks {
    Unstaged,
    Staged,
    Diff,
}

pub struct StatusTab {
    pub line_in_file: u16,
    pub line_in_folder_unstaged: u16,
    pub line_in_folder_staged: u16,
    pub focused_block: StatusBlocks,
}

impl StatusTab {
    pub fn draw(&self, frame: &mut Frame, content: Rect) {
        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(content);
        let [top_left, bottom_left] = Layout::vertical([Constraint::Fill(1); 2]).areas(left);

        self.draw_diff(frame, right);
        self.draw_unstaged(frame, top_left);
        self.draw_staged(frame, bottom_left);
    }

    fn draw_diff(&self, frame: &mut Frame, pos: Rect) {
        let text = fs::read_to_string("Cargo.lock")
            .unwrap_or_else(|_| "Error: Could not read file".to_string());

        let diff = Paragraph::new(text)
            .style(Style::default())
            .block(Block::bordered().title("Diff"))
            .scroll((self.line_in_file, 0));

        frame.render_widget(diff, pos);
    }

    fn draw_unstaged(&self, frame: &mut Frame, pos: Rect) {
        let files: Vec<GitFile> =
            get_files(".", TypeStaged::Unstaged).expect("Error on unstaged file");

        let items: Vec<ListItem> = if files.is_empty() {
            vec![ListItem::new("No unstaged changes")]
        } else {
            files
                .iter()
                .map(|file| {
                    let style = match file.status {
                        'm' => Style::default().fg(Color::Yellow), // Modified
                        'd' => Style::default().fg(Color::Red),    // Deleted
                        'r' => Style::default().fg(Color::Blue),   // Untracked
                        'n' => Style::default().fg(Color::Green),  // Added
                        _ => Style::default(),
                    };

                    ListItem::new(format!("{} {}", file.status, file.filename)).style(style)
                })
                .collect()
        };

        let unstaged_list = List::new(items)
            .block(Block::bordered().title(format!("Unstaged ({})", files.len())))
            .highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_widget(unstaged_list, pos);
    }

    fn draw_staged(&self, frame: &mut Frame, pos: Rect) {
        let files: Vec<GitFile> =
            get_files(".", TypeStaged::Staged).expect("Error on unstaged file");

        let items: Vec<ListItem> = if files.is_empty() {
            vec![ListItem::new("No unstaged changes")]
        } else {
            files
                .iter()
                .map(|file| {
                    let style = match file.status {
                        'm' => Style::default().fg(Color::Yellow), // Modified
                        'd' => Style::default().fg(Color::Red),    // Deleted
                        'r' => Style::default().fg(Color::Blue),   // Untracked
                        'n' => Style::default().fg(Color::Green),  // Added
                        _ => Style::default(),
                    };

                    ListItem::new(format!("{} {}", file.status, file.filename)).style(style)
                })
                .collect()
        };

        let unstaged_list = List::new(items)
            .block(Block::bordered().title(format!("Unstaged ({})", files.len())))
            .highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_widget(unstaged_list, pos);
    }
}
