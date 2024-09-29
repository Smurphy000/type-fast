use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::trace;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // todo may want to define this globally
    let alphabet = (b'A'..=b'z') // Start as u8
        .filter_map(|c| {
            let c = c as char; // Convert to char
            if c.is_alphabetic() {
                Some(c)
            } else {
                None
            } // Filter only alphabetic chars
        })
        .collect::<Vec<_>>();
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        // KeyCode::Char('c') | KeyCode::Char('C') => {
        //     if key_event.modifiers == KeyModifiers::CONTROL {
        //         app.quit();
        //     }
        // }
        KeyCode::Char(ch) => {
            if alphabet.contains(&ch) {
                // todo all input from user for all alphabetic character while in typing mode.
                trace!(target:"Input", "User input char {}", ch);
                app.input_letter = Some(ch);
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
