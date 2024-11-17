use crate::utils::Blocks;
use crossterm::event::KeyCode;

pub struct App {
    pub should_quit: bool,
    pub current_block: Blocks,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            current_block: Blocks::First,
        }
    }

    // update widgets
    pub fn on_tick(&mut self) {}

    // handle key events
    pub fn on_key(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('1') => self.current_block = Blocks::First,
            KeyCode::Char('2') => self.current_block = Blocks::Second,
            KeyCode::Char('3') => self.current_block = Blocks::Third,
            KeyCode::Char('4') => self.current_block = Blocks::Fourth,
            KeyCode::Char('5') => self.current_block = Blocks::Fifth,
            _ => {}
        }
    }
}
