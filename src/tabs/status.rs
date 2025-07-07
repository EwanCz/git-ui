use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::git::{get_file_diff, get_files, Git, GitFile, TypeStaged};

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
    pub nb_unstaged_file: u16,
    pub nb_staged_file: u16,
    pub filepath_diff: String,
}

impl StatusTab {
    pub fn draw(&mut self, frame: &mut Frame, content: Rect, git: &Git) {
        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(content);
        let [top_left, bottom_left] = Layout::vertical([Constraint::Fill(1); 2]).areas(left);

        let staged_files: Vec<GitFile> =
            get_files(git, TypeStaged::Staged).expect("Error on staged file");
        let unstaged_files: Vec<GitFile> =
            get_files(git, TypeStaged::Unstaged).expect("Error on unstaged file");
        let file_to_read = match self.focused_block {
            StatusBlocks::Diff => self.filepath_diff.clone(),
            StatusBlocks::Staged => {
                self.get_selected_file_path(staged_files.clone(), self.line_in_folder_staged)
            }
            StatusBlocks::Unstaged => {
                self.get_selected_file_path(unstaged_files.clone(), self.line_in_folder_unstaged)
            }
        };

        self.nb_unstaged_file = unstaged_files.len() as u16;
        self.nb_staged_file = staged_files.len() as u16;

        self.draw_diff(frame, right, file_to_read);
        self.draw_unstaged(frame, top_left, unstaged_files);
        self.draw_staged(frame, bottom_left, staged_files);
    }

    fn get_selected_file_path(&mut self, files: Vec<GitFile>, pos: u16) -> String {
        if !files.is_empty() {
            self.filepath_diff = files[pos as usize].filename.clone();
            files[pos as usize].filename.clone()
        } else {
            String::new()
        }
    }

    fn draw_diff(&self, frame: &mut Frame, pos: Rect, file: String) {
        let text = match get_file_diff(&file) {
            // Retourne Text avec styles
            Ok(styled_text) => styled_text,
            Err(e) => Text::from(e),
        };

        let diff =
            Paragraph::new(text)
                .style(Style::default())
                .block(self.make_status_block(
                    self.focused_block == StatusBlocks::Diff,
                    "Diff".to_string(),
                ))
                .scroll((self.line_in_file, 0));

        frame.render_widget(Clear, pos);
        frame.render_widget(diff, pos);
    }

    fn draw_unstaged(&self, frame: &mut Frame, pos: Rect, files: Vec<GitFile>) {
        let mut items: Vec<ListItem> = if files.is_empty() {
            vec![ListItem::new("No unstaged changes")]
        } else {
            files
                .iter()
                .skip(self.line_in_folder_unstaged.into())
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
        if self.focused_block == StatusBlocks::Unstaged {
            items[0] = items[0].clone().on_dark_gray();
        }
        let unstaged_list = List::new(items)
            .block(self.make_status_block(
                self.focused_block == StatusBlocks::Unstaged,
                format!("Unstaged ({})", files.len()),
            ))
            .highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_widget(unstaged_list, pos);
    }

    fn draw_staged(&self, frame: &mut Frame, pos: Rect, files: Vec<GitFile>) {
        let mut items: Vec<ListItem> = if files.is_empty() {
            vec![ListItem::new("No staged changes")]
        } else {
            files
                .iter()
                .skip(self.line_in_folder_staged.into())
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

        if self.focused_block == StatusBlocks::Staged {
            items[0] = items[0].clone().on_dark_gray();
        }

        let staged_list = List::new(items)
            .block(self.make_status_block(
                self.focused_block == StatusBlocks::Staged,
                format!("Staged ({})", files.len()),
            ))
            .highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_widget(staged_list, pos);
    }

    fn make_status_block(&self, focus: bool, title: String) -> Block {
        if focus {
            Block::bordered().title(title).bold()
        } else {
            Block::bordered().title(title)
        }
    }

    pub fn change_block(&mut self, code: KeyCode) {
        if self.focused_block != StatusBlocks::Diff && code == KeyCode::Right {
            self.focused_block = StatusBlocks::Diff;
            return;
        }
        if self.focused_block == StatusBlocks::Diff && code == KeyCode::Left {
            self.focused_block = StatusBlocks::Unstaged;
            return;
        }
        if self.focused_block == StatusBlocks::Unstaged && code == KeyCode::Down {
            self.focused_block = StatusBlocks::Staged;
            return;
        }
        if self.focused_block == StatusBlocks::Staged && code == KeyCode::Up {
            self.focused_block = StatusBlocks::Unstaged;
        }
    }

    pub fn scroll_down(&mut self) {
        match self.focused_block {
            StatusBlocks::Diff => self.line_in_file += 1,
            StatusBlocks::Unstaged => {
                self.line_in_file = 0;
                if self.line_in_folder_unstaged + 1 < self.nb_unstaged_file {
                    self.line_in_folder_unstaged += 1;
                }
            }
            StatusBlocks::Staged => {
                self.line_in_file = 0;
                if self.line_in_folder_staged + 1 < self.nb_staged_file {
                    self.line_in_folder_staged += 1;
                }
            }
        }
    }

    pub fn scroll_up(&mut self) {
        match self.focused_block {
            StatusBlocks::Diff => {
                if self.line_in_file > 0 {
                    self.line_in_file -= 1;
                }
            }
            StatusBlocks::Unstaged => {
                if self.line_in_folder_unstaged > 0 {
                    self.line_in_folder_unstaged -= 1;
                }
            }
            StatusBlocks::Staged => {
                if self.line_in_folder_staged > 0 {
                    self.line_in_folder_staged -= 1;
                }
            }
        }
    }

    pub fn handle_pos_in_blocks(&mut self, block: StatusBlocks) {
        if StatusBlocks::Unstaged == block && self.line_in_folder_unstaged != 0 {
            self.line_in_folder_unstaged -= 1;
        }
        if StatusBlocks::Staged == block && self.line_in_folder_staged != 0 {
            self.line_in_folder_staged -= 1;
        }
    }
}
