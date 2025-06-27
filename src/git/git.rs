use crossterm::event::{KeyCode, KeyEvent};
use git2::{Error as GitError, Repository};
use std::path::Path;

use crate::git::Commit;

#[derive(PartialEq)]
pub enum CommitMode {
    Normal,
    Commit,
}

use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

pub struct Git {
    pub repo: Repository,
    pub input: String,
    pub character_index: usize,
    pub commit_mode: CommitMode,
    pub messages: Vec<String>,
}

impl Git {
    pub fn add(&self, filepath: &str) -> Result<(), GitError> {
        let mut index = self.repo.index()?;
        index.add_path(Path::new(filepath))?;
        index.write()?;
        Ok(())
    }

    pub fn restore_staged(&self, filepath: &str) -> Result<(), GitError> {
        let head = self.repo.head()?;
        let head_commit = head.peel_to_commit()?;
        let head_tree = head_commit.tree()?;

        let mut index = self.repo.index()?;

        match head_tree.get_path(Path::new(filepath)) {
            Ok(tree_entry) => {
                let blob = self.repo.find_blob(tree_entry.id())?;
                index.add_frombuffer(
                    &git2::IndexEntry {
                        ctime: git2::IndexTime::new(0, 0),
                        mtime: git2::IndexTime::new(0, 0),
                        dev: 0,
                        ino: 0,
                        mode: tree_entry.filemode() as u32,
                        uid: 0,
                        gid: 0,
                        file_size: blob.size() as u32,
                        id: tree_entry.id(),
                        flags: filepath.len() as u16,
                        flags_extended: 0,
                        path: filepath.to_string().into(),
                    },
                    blob.content(),
                )?;
            }
            Err(_) => {
                index.remove_path(Path::new(filepath))?;
            }
        }

        index.write()?;
        Ok(())
    }

    pub fn change_commit_mode(&mut self) {
        self.commit_mode = match self.commit_mode {
            CommitMode::Normal => CommitMode::Commit,
            CommitMode::Commit => CommitMode::Normal,
        }
    }

    pub fn draw_commit(&self, frame: &mut Frame, content: Rect) {
        let block = Block::bordered().title("Commit");
        let text = Paragraph::new(self.input.clone()).centered().block(block);

        let vertical = Layout::vertical([Constraint::Percentage(20)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
        let [content] = vertical.areas(content);
        let [content] = horizontal.areas(content);

        frame.render_widget(Clear, content);
        frame.render_widget(text, content);
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn commit_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.change_commit_mode(),
            KeyCode::Char(to_insert) => self.enter_char_commit(to_insert),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Enter => {
                let _ = self.git_commit();
                self.commit_mode = CommitMode::Normal
            }
            _ => {}
        }
    }
}
