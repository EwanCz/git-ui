use crossterm::event::{KeyCode, KeyEvent};
use git2::{Error as GitError, Repository};
use ratatui::{
    layout::{Constraint, Flex, Layout, Position, Rect},
    widgets::{Block, Clear, Paragraph},
    Frame,
};
use std::{path::Path, sync::mpsc, thread};

use crate::git::{execute_push, get_repository, Commit, CommitMode, PushMode};

pub struct Git {
    pub repo: Repository,
    pub branch: String,
    pub input: String,
    pub character_index: usize,
    pub messages: Vec<String>,
    pub commit_mode: CommitMode,
    pub push_mode: PushMode,
    pub push_message: String,
    pub push_process: bool,
    pub rx_push: Option<mpsc::Receiver<String>>,
}

impl Git {
    pub fn new(repository: Repository, current_branch: String) -> Self {
        Git {
            repo: repository,
            branch: current_branch,
            input: String::new(),
            character_index: 0,
            commit_mode: CommitMode::Normal,
            messages: Vec::new(),
            push_mode: PushMode::Normal,
            push_message: String::from("Are you sure you want to push your work ?"),
            push_process: false,
            rx_push: None,
        }
    }

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
        let text = Paragraph::new(self.input.clone()).block(block);

        let vertical = Layout::vertical([Constraint::Max(4)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
        let [content] = vertical.areas(content);
        let [content] = horizontal.areas(content);

        frame.set_cursor_position(Position::new(
            content.x + 1 + self.character_index as u16,
            content.y + 1,
        ));
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
                if self.push_process {
                    return;
                }
                let (tx, rx) = mpsc::channel();
                self.rx_push = Some(rx);
                self.push_process = true;
                self.push_message = "🔄 Initializing push...".to_string();

                let repo = match get_repository() {
                    Ok(value) => value,
                    Err(_e) => {
                        self.push_message = "❌ Push failed: Can't get actual repo".to_string();
                        return;
                    }
                };
                let branch = self.branch.clone();

                thread::spawn(move || match execute_push(repo, branch, tx.clone()) {
                    Ok(value) => {
                        tx.send(value).unwrap();
                    }
                    Err(error) => {
                        tx.send(format!("❌ Push failed: {}", error.message()))
                            .unwrap();
                    }
                });
            }
            _ => {}
        }
    }

    pub fn update_push_status(&mut self) {
        if let Some(rx) = &self.rx_push {
            // Récupérer TOUS les messages disponibles
            match rx.try_recv() {
                Ok(message) => {
                    self.push_message = message.clone();
                    if message.starts_with("✅") || message.starts_with("❌") {
                        self.push_process = false;
                        self.rx_push = None;
                    }
                }
                Err(_e) => {}
            }
        }
    }
}
