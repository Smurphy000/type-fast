use std::{error, fmt};

use ratatui::widgets::{ListItem, ListState};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Pages {
    Menu,
    Typing,
    Stats,
    Historical,
}

#[derive(Debug)]
pub enum MenuOptions {
    Type,
    Options,
    Credits,
    Quit,
}

impl fmt::Display for MenuOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuOptions::Type => write!(f, "Type"),
            MenuOptions::Options => write!(f, "Options"),
            MenuOptions::Credits => write!(f, "Credits"),
            MenuOptions::Quit => write!(f, "Quit"),
        }
    }
}

#[derive(Debug)]
pub struct Menu {
    pub options: Vec<MenuOptions>,
    pub current_selection: ListState,
}

impl Menu {
    fn new() -> Self {
        Self {
            options: vec![
                MenuOptions::Type,
                MenuOptions::Options,
                MenuOptions::Credits,
                MenuOptions::Quit,
            ],
            current_selection: ListState::default(),
        }
    }

    pub fn select_none(&mut self) {
        self.current_selection.select(None);
    }

    pub fn select_next(&mut self) {
        self.current_selection.select_next();
    }
    pub fn select_previous(&mut self) {
        self.current_selection.select_previous();
    }

    pub fn select_first(&mut self) {
        self.current_selection.select_first();
    }

    pub fn select_last(&mut self) {
        self.current_selection.select_last();
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub current_page: Pages,
    pub menu: Menu,
    pub current_words: Vec<String>,
    pub current_letter: Option<char>,
    pub input_letter: Option<char>,
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
            current_letter: None,
            input_letter: None,
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
