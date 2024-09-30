pub mod menu;
pub use menu::{Menu, MenuOptions};

// All possible pages the user could be shown
#[derive(Debug)]
pub enum Pages {
    Menu,
    Typing,
    Stats,
    Historical,
}
