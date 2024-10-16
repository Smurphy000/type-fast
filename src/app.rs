mod pages;

use log::trace;
use pages::{
    pause::{Pause, PauseOptions},
    typing::TypingStats,
};
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
    pub pause_popup: Pause,
    pub paused: bool,
    pub previous_stats: TypingStats,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            running: true,
            current_page: Pages::Menu,
            menu: Menu::new(),
            typing: Typing::new(),
            pause_popup: Pause::new(),
            paused: false,
            previous_stats: TypingStats {
                wpm: 0.0,
                accuracy: 0.0,
                awpm: 0.0,
            },
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

    pub fn pause(&mut self) {
        self.paused = true;
        self.pause_popup.select_first();
        self.current_page = Pages::Pause;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
        self.current_page = Pages::Typing;
    }

    pub fn select_pause_option(&mut self) {
        match self.pause_popup.current_selection.selected() {
            Some(x) => {
                let selected = &self.pause_popup.options[x];
                match selected {
                    PauseOptions::Resume => self.unpause(),

                    PauseOptions::Quit => {
                        self.unpause();
                        self.current_page = Pages::Menu
                    }
                }
            }
            None => todo!(),
        }
    }

    pub fn select_menu_option(&mut self) {
        match self.menu.current_selection.selected() {
            Some(x) => {
                let selected = &self.menu.options[x];
                match selected {
                    MenuOptions::Type => self.setup_typing(),
                    MenuOptions::Options => todo!(),
                    MenuOptions::Credits => todo!(),
                    MenuOptions::Quit => self.quit(),
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
        self.previous_stats = self.typing.calculate_statistics();
        self.setup_typing();
    }
}
