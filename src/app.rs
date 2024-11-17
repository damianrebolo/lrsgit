pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            should_quit: false,
        }
    }

    // update widgets
    pub fn on_tick(&mut self) {}

    // handle key events
    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => self.should_quit = true,
            _ => {}
        }
    }
}
