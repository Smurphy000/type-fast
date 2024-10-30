pub mod menu;
pub use menu::{Menu, MenuOptions};
pub mod typing;
pub use typing::{PromptSettings, Typing};
pub mod pause;

// All possible pages the user could be shown
#[derive(Debug)]
pub enum Pages {
    Menu,
    Typing,
    Stats,
    Pause,
}
