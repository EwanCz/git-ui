use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use git2::BranchType;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::{
    git::{Branch, Git},
    popup::Popup,
    tabs::mover::{Move, DIRECTION},
};

#[derive(PartialEq)]
pub enum BranchBlock {
    Local,
    Remote,
}

pub struct BranchTab {
    pub pos_local_branches: u16,
    pub pos_remote_branches: u16,
    pub nb_remote_branch: u16,
    pub nb_local_branch: u16,
    pub newbranch_popup: Popup,
    pub focused_block: BranchBlock,
}

impl BranchTab {
    pub fn new() -> Self {
        BranchTab {
            pos_local_branches: 0,
            pos_remote_branches: 0,
            nb_local_branch: 0,
            nb_remote_branch: 0,
            newbranch_popup: Popup::new(),
            focused_block: BranchBlock::Local,
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, git: &mut Git) {
        if key_event.modifiers == KeyModifiers::CONTROL {
            self.change_block(key_event.code);
            return;
        }
        match key_event.code {
            KeyCode::Up => self.scroll_up(),
            KeyCode::Down => self.scroll_down(),
            KeyCode::Char('c') => {
                let (branchtype, pos): (BranchType, usize) = match self.focused_block {
                    BranchBlock::Local => (BranchType::Local, self.pos_local_branches as usize),
                    BranchBlock::Remote => (BranchType::Remote, self.pos_remote_branches as usize),
                };
                let _ = git.branch.checkout(branchtype, pos, &git.repo);
                self.reset_branch(git);
            }
            KeyCode::Char('d') => {
                if self.focused_block == BranchBlock::Remote {
                    return;
                }
                let branch_name: String =
                    git.branch.local_branches[self.pos_local_branches as usize].clone();
                let _ = git.branch.delete_branch(&branch_name, &git.repo);
                self.reset_branch(git);
            }
            KeyCode::Char('n') => self.newbranch_popup.activated = true,
            _ => {}
        }
    }

    pub fn newbranch_key_event(&mut self, key_event: KeyEvent, git: &mut Git) {
        match key_event.code {
            KeyCode::Esc => self.newbranch_popup.activated = false,
            KeyCode::Char(to_insert) => self.newbranch_popup.enter_char(to_insert),
            KeyCode::Left => self.newbranch_popup.move_cursor_left(),
            KeyCode::Right => self.newbranch_popup.move_cursor_right(),
            KeyCode::Backspace => self.newbranch_popup.delete_char(),
            KeyCode::Enter => {
                let _ = git.branch.create_branch("test", &git.repo);
                self.reset_branch(git);
                self.newbranch_popup.input = String::new();
                self.newbranch_popup.activated = false
            }
            _ => {}
        }
    }

    fn reset_branch(&mut self, git: &mut Git) {
        self.pos_local_branches = 0;
        self.pos_remote_branches = 0;
        git.branch = Branch::new(&git.repo);
        self.nb_local_branch = git.branch.local_branches.len() as u16;
        self.nb_remote_branch = git.branch.remote_branches.len() as u16;
    }

    pub fn draw(&self, frame: &mut Frame, content: Rect, git: &Git) {
        let [top, bottom] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(content);
        let [bottom_left, bottom_right] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Fill(1)]).areas(bottom);

        self.draw_local_branches(frame, bottom_left, git);
        self.draw_remote_branches(frame, bottom_right, git);
        self.draw_current_branch(frame, top, git);
    }

    fn draw_current_branch(&self, frame: &mut Frame, area: Rect, git: &Git) {
        let zone = Paragraph::new(format!("Current branch: {}", git.branch.current)).centered();
        frame.render_widget(zone, area);
    }

    fn draw_local_branches(&self, frame: &mut Frame, area: Rect, git: &Git) {
        let block = self.make_status_block(
            self.focused_block == BranchBlock::Local,
            "Branches".to_string(),
        );
        let paragraph = Paragraph::new(git.branch.local_branches.clone().join("\n"))
            .centered()
            .scroll((self.pos_local_branches, 0))
            .block(block);
        frame.render_widget(paragraph, area);
    }

    fn draw_remote_branches(&self, frame: &mut Frame, area: Rect, git: &Git) {
        let block = self.make_status_block(
            self.focused_block == BranchBlock::Remote,
            "Remote".to_string(),
        );

        let paragraph = Paragraph::new(git.branch.remote_branches.clone().join("\n"))
            .centered()
            .scroll((self.pos_remote_branches, 0))
            .block(block);
        frame.render_widget(paragraph, area);
    }
}

impl Default for BranchTab {
    fn default() -> Self {
        BranchTab::new()
    }
}

impl Move for BranchTab {
    fn scroll_up(&mut self) {
        match self.focused_block {
            BranchBlock::Local => {
                if self.pos_local_branches > 0 {
                    self.pos_local_branches -= 1;
                }
            }
            BranchBlock::Remote => {
                if self.pos_remote_branches > 0 {
                    self.pos_remote_branches -= 1;
                }
            }
        }
    }

    fn scroll_down(&mut self) {
        match self.focused_block {
            BranchBlock::Remote => {
                if self.pos_remote_branches + 1 < self.nb_remote_branch {
                    self.pos_remote_branches += 1;
                }
            }
            BranchBlock::Local => {
                if self.pos_local_branches + 1 < self.nb_local_branch {
                    self.pos_local_branches += 1;
                }
            }
        }
    }

    fn change_block(&mut self, code: KeyCode) {
        if !DIRECTION.contains(&code) {
            return;
        }
        match code {
            KeyCode::Left => self.focused_block = BranchBlock::Local,
            KeyCode::Right => self.focused_block = BranchBlock::Remote,
            _ => {}
        }
    }
}
