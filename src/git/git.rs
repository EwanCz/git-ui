use crossterm::event::{KeyCode, KeyEvent};
use git2::{Error as GitError, Repository};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Clear, Paragraph},
    Frame,
};
use std::path::Path;

use crate::git::{Commit, CommitMode, Push, PushMode};

pub struct Git {
    pub repo: Repository,
    pub branch: String,
    pub input: String,
    pub character_index: usize,
    pub messages: Vec<String>,
    pub commit_mode: CommitMode,
    pub push_mode: PushMode,
    pub push_message: String,
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

    pub fn draw_push(&self, frame: &mut Frame, content: Rect) {
        let block = Block::bordered().title("Push");
        let text = Paragraph::new(self.push_message.clone())
            .centered()
            .block(block);

        let vertical = Layout::vertical([Constraint::Percentage(15)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(40)]).flex(Flex::Center);
        let [content] = vertical.areas(content);
        let [content] = horizontal.areas(content);

        frame.render_widget(Clear, content);
        frame.render_widget(text, content);
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

    pub fn push_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.push_mode = PushMode::Normal;
                self.push_message = String::from("Are you sure you want to push your work ?");
            }
            KeyCode::Enter => {
                self.push_message = self
                    .execute_push()
                    .unwrap_or_else(|git_error| git_error.message().to_string());
            }
            _ => {}
        }
    }
}
