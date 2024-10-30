use crate::{
    app::{App, AppResult},
    cli::Cli,
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};
use dirs;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use clap::Parser;

use log::*;
use tui_logger::*;

pub mod app;
pub mod cli;
pub mod config;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
    init_logger(LevelFilter::Trace).unwrap();
    set_default_level(LevelFilter::Trace);

    let cli = Cli::parse();
    // TODO load config
    if cli.config == "" {
        // load from default, otherwise specified
    }

    trace!(target: "main", "config dir: {:?}", dirs::config_dir());

    trace!(target: "main", "skip {}", cli.skip_menu);

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(h, w) => app.resize(h, w),
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
