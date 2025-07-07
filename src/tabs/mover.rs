use crossterm::event::KeyCode;
use ratatui::{style::Stylize, widgets::Block};

pub trait Move {
    fn scroll_down(&mut self);

    fn scroll_up(&mut self);

    fn change_block(&mut self, code: KeyCode);

    fn make_status_block(&self, focus: bool, title: String) -> Block {
        if focus {
            Block::bordered().title(title).bold()
        } else {
            Block::bordered().title(title)
        }
    }
}
