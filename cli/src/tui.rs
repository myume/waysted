use std::io;

use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if matches!(event::read()?, Event::Key(_)) {
            self.exit = true;
        }
        Ok(())
    }
}
