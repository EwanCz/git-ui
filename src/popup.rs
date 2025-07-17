use ratatui::{
    layout::{Constraint, Flex, Layout, Position, Rect},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

pub struct Popup {
    pub input: String,
    pub character_index: usize,
    pub messages: Vec<String>,
}

impl Popup {
    pub fn new() -> Self {
        Popup {
            input: String::new(),
            character_index: 0,
            messages: Vec::new(),
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
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

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn draw_popup(&self, frame: &mut Frame, content: Rect, name_block: &str) {
        let block = Block::bordered().title(name_block);
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
}
