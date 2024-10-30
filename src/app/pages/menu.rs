use std::fmt;

use ratatui::widgets::ListState;

// Available menu options when first running the application
#[derive(Debug)]
pub enum MenuOptions {
    Type,
    Quit,
}

// We implement Display for MenuOptions to allow for use of the to_string fn. Bit of a hack
impl fmt::Display for MenuOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuOptions::Type => write!(f, "Type"),
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
    pub fn new() -> Self {
        // initialize state as the first item in the menu
        let mut state = ListState::default();
        state.select_first();
        Self {
            options: vec![MenuOptions::Type, MenuOptions::Quit],
            current_selection: state,
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
