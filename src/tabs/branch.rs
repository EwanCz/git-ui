use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{List, Paragraph},
    Frame,
};

use crate::git::Git;

pub struct BranchTab {
    pub value: u32,
}

impl BranchTab {
    pub fn new() -> Self {
        BranchTab { value: 0 }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('w') => println!("tmp"),
            KeyCode::Char('q') => println!("tmp2"),
            _ => {}
        }
    }

    pub fn draw(&self, frame: &mut Frame, content: Rect, git: &Git) {
        let [top, bottom] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(content);

        self.draw_list_of_branche(frame, bottom, git);
        self.draw_current_branch(frame, top, git);
    }

    fn draw_current_branch(&self, frame: &mut Frame, area: Rect, git: &Git) {
        let zone = Paragraph::new(format!("Current branch: {}", git.branch.current)).centered();
        frame.render_widget(zone, area);
    }

    fn draw_list_of_branche(&self, frame: &mut Frame, area: Rect, git: &Git) {
        let items: List = git.branch.branches.clone().into_iter().collect();
        frame.render_widget(items, area);
    }
}

impl Default for BranchTab {
    fn default() -> Self {
        BranchTab::new()
    }
}
