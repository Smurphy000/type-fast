mod pages;

pub use pages::{Menu, MenuOptions, Pages};

use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub current_page: Pages,
    pub menu: Menu,
    pub current_words: Vec<String>,
    pub current_letter: String,
    pub input_letter: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_page: Pages::Menu,
            menu: Menu::new(),
            current_words: vec![
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
            ],
            current_letter: String::from(""),
            input_letter: String::from(""),
        }
    }
}

impl App {
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
                    MenuOptions::Type => self.current_page = Pages::Typing,
                    MenuOptions::Options => todo!(),
                    MenuOptions::Credits => todo!(),
                    MenuOptions::Quit => todo!(),
                }
            }
            None => todo!(),
        }
    }
}
