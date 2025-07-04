use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;

use std::cell::RefCell;

use crate::{
    git::{CommitMode, Git, PushMode},
    pages::Pages,
    tabs::{StatusBlocks, StatusTab},
};

pub struct App {
    pub exit: bool,
    pub page: Pages,
    pub status_page: RefCell<StatusTab>,
    pub git: Git,
}

const PAGESNAME: [&str; 3] = [" [1 status] ", " [2 info] ", " [3 Config] "];
const DIRECTION: [KeyCode; 4] = [KeyCode::Up, KeyCode::Left, KeyCode::Right, KeyCode::Down];

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [_header, content] = Layout::vertical([
            Constraint::Length(2), // Header height (adjust as needed)
            Constraint::Fill(1),   // Rest for blocks
        ])
        .areas(frame.area());

        if self.page == Pages::StatusPAGE {
            self.status_page
                .borrow_mut()
                .draw(frame, content, &self.git);
            if self.git.commit_mode == CommitMode::Commit {
                self.git.draw_commit(frame, content);
            }
            if self.git.push_mode == PushMode::Push {
                self.git.draw_push(frame, content);
            }
        }

        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.git.push_mode == PushMode::Push {
            self.git.push_key_event(key_event);
            return;
        }
        if self.git.commit_mode == CommitMode::Commit {
            self.git.commit_key_event(key_event);
            return;
        }
        if key_event.modifiers == KeyModifiers::CONTROL {
            self.change_block(key_event.code);
            return;
        }
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Char(char @ '1'..='3') => {
                let nb: u32 = char.to_digit(10).unwrap();
                self.page = self.page.change_page(nb - 1);
            }
            KeyCode::Down => self.scroll_down(),
            KeyCode::Up => self.scroll_up(),
            KeyCode::Char('a') => {
                if self.page == Pages::StatusPAGE
                    && self.status_page.borrow_mut().focused_block == StatusBlocks::Unstaged
                {
                    let _ = self.git.add(&self.status_page.borrow_mut().filepath_diff);
                }
            }
            KeyCode::Char('r') => {
                if self.page == Pages::StatusPAGE
                    && self.status_page.borrow_mut().focused_block == StatusBlocks::Staged
                {
                    let _ = self
                        .git
                        .restore_staged(&self.status_page.borrow_mut().filepath_diff);
                }
            }
            KeyCode::Char('c') => {
                if self.page == Pages::StatusPAGE {
                    self.git.change_commit_mode();
                }
            }
            KeyCode::Char('p') => {
                if self.page == Pages::StatusPAGE {
                    self.git.push_mode = PushMode::Push;
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true
    }

    fn scroll_up(&mut self) {
        if self.page == Pages::StatusPAGE {
            self.status_page.borrow_mut().scroll_up();
        }
    }

    fn scroll_down(&mut self) {
        if self.page == Pages::StatusPAGE {
            self.status_page.borrow_mut().scroll_down();
        }
    }

    fn change_block(&mut self, code: KeyCode) {
        if !DIRECTION.contains(&code) {
            return;
        }
        if self.page == Pages::StatusPAGE {
            self.status_page.borrow_mut().change_block(code);
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create header area (top 3 lines)
        let header_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 3,
        };

        // Create header with page navigation
        let mut pages: [Span; 3] = [
            Span::raw(PAGESNAME[0]),
            Span::raw(PAGESNAME[1]),
            Span::raw(PAGESNAME[2]),
        ];

        for i in 0..=2 {
            if i == self.page.to_index() {
                pages[i] = Span::styled(
                    PAGESNAME[i],
                    Style::default().bg(Color::Red).fg(Color::White),
                );
            }
        }

        let header_text = Line::from(vec![pages[0].clone(), pages[1].clone(), pages[2].clone()]);

        Paragraph::new(header_text)
            .alignment(Alignment::Center)
            .render(header_area, buf);
    }
}
