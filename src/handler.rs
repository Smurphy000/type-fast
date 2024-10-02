use crate::app::{App, AppResult, Pages};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::trace;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // todo may want to define this globally

    match app.current_page {
        Pages::Menu => handle_menu(key_event, app),
        Pages::Typing => handle_typing(key_event, app),
        Pages::Stats => todo!(),
        Pages::Historical => todo!(),
    }

    Ok(())
}

fn handle_menu(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }

        KeyCode::Char('h') | KeyCode::Left => app.menu.select_none(),
        KeyCode::Char('j') | KeyCode::Down => app.menu.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.menu.select_previous(),
        KeyCode::Char('g') | KeyCode::Home => app.menu.select_first(),
        KeyCode::Char('G') | KeyCode::End => app.menu.select_last(),
        KeyCode::Enter => app.select_menu_option(),

        _ => {}
    }
}

fn handle_typing(key_event: KeyEvent, app: &mut App) {
    // TODO could extend alphabet to include other necessary characters
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

    let mut prompt_complete = false;
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }

        KeyCode::Char(' ') => {
            prompt_complete = app.typing.input('•');

            app.input_letter.push(' ');
        }

        KeyCode::Char(ch) => {
            if alphabet.contains(&ch) {
                // todo all input from user for all alphabetic character while in typing mode.
                trace!(target:"Input", "User input char {}", ch);
                prompt_complete = app.typing.input(ch);
                app.input_letter.push(ch);
            }
        }

        // Other handlers you could add here.
        _ => {}
    }

    app.typing.construct_text();
    if prompt_complete {
        app.new_prompt();
    }
}
