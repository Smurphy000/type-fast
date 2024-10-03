mod pages;

pub use pages::{Menu, MenuOptions, Pages, Typing};

use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    pub current_page: Pages,
    pub menu: Menu,
    pub typing: Typing<'a>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            running: true,
            current_page: Pages::Menu,
            menu: Menu::new(),
            typing: Typing::new(),
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select_menu_option(&mut self) {
        match self.menu.current_selection.selected() {
            Some(x) => {
                let selected = &self.menu.options[x];
                match selected {
                    MenuOptions::Type => self.setup_typing(),
                    MenuOptions::Options => todo!(),
                    MenuOptions::Credits => todo!(),
                    MenuOptions::Quit => todo!(),
                }
            }
            None => todo!(),
        }
    }

    fn setup_typing(&mut self) {
        self.current_page = Pages::Typing;
        self.typing = Typing::new();
    }

    pub fn new_prompt(&mut self) {
        self.setup_typing();
    }
}
