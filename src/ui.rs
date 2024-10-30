use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Clear, HighlightSpacing, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};
use tui_big_text::BigText;

use crate::app::{App, Pages};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    let area = frame.area();
    let mut a_buf: Buffer = Buffer::empty(area);

    match app.current_page {
        Pages::Menu => render_menu(frame, app, area, &mut a_buf),
        Pages::Typing => render_typing(frame, app, area),
        Pages::Stats => todo!(),
        Pages::Pause => render_typing(frame, app, area),
    }
}

fn render_menu(frame: &mut Frame, app: &mut App, smart_area: Rect, _buf: &mut Buffer) {
    let [top_area, bottom_area] =
        { Layout::vertical([Constraint::Fill(30), Constraint::Fill(70)]).areas(smart_area) };

    let list_items: Vec<ListItem> = app
        .menu
        .options
        .iter()
        .map(|x| ListItem::from(x.to_string()))
        .collect();
    let list = List::new(list_items)
        .highlight_style(Style::default())
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always)
        .block(Block::new().bg(Color::Black).padding(Padding::new(
            bottom_area.width / 4,
            bottom_area.width / 4,
            0,
            0,
        )));

    render_blank(frame, top_area);

    frame.render_widget(
        BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Full)
            .style(Style::new().white().bg(Color::Black))
            .lines(vec!["Type Fast!".white().into()])
            .alignment(Alignment::Center)
            .build(),
        top_area,
    );
    frame.render_stateful_widget(list, bottom_area, &mut app.menu.current_selection);
}

fn render_typing(frame: &mut Frame, app: &mut App, smart_area: Rect) {
    let [top_area, prompt_area, bottom_area] = {
        Layout::vertical([
            Constraint::Fill(20),
            Constraint::Fill(60),
            Constraint::Fill(20),
        ])
        .areas(smart_area)
    };

    if !app.prompt_settings.borrow().zen {
        render_settings(frame, app, top_area);
    } else {
        render_blank(frame, top_area);
    }

    // TODO wrap on word end instead of characters, this may not be possible for my current implementation
    frame.render_widget(
        Paragraph::new(app.typing.text.clone())
            .wrap(Wrap { trim: true })
            .block(
                Block::bordered()
                    .hidden()
                    .padding(Padding::new(10, 10, top_area.height / 2, 0))
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default().fg(Color::Black).bg(Color::Black))
            .centered(),
        prompt_area,
    );

    // display the previous prompts stats
    if !app.prompt_settings.borrow().zen {
        render_stats(frame, app, bottom_area);
    } else {
        render_blank(frame, bottom_area);
    }

    if app.paused {
        pause_popup(frame, smart_area);
    }
}

fn render_settings(frame: &mut Frame, app: &mut App, smart_area: Rect) {
    let [one, two, three, four] = {
        Layout::horizontal([
            Constraint::Fill(25),
            Constraint::Fill(25),
            Constraint::Fill(25),
            Constraint::Fill(25),
        ])
        .areas(smart_area)
    };
    let settings = app.typing.settings.borrow();
    frame.render_widget(
        Paragraph::new(vec![Line::from(format!("WC: {}", settings.wc))])
            .block(
                Block::new()
                    .padding(Padding::new(0, 0, smart_area.height / 2, 0))
                    .bg(Color::Black),
            )
            .alignment(Alignment::Right),
        one,
    );
    frame.render_widget(
        Paragraph::new(vec![Line::from(format!(
            "Caps: {}",
            settings.capitalization
        ))])
        .block(
            Block::new()
                .padding(Padding::new(0, 0, smart_area.height / 2, 0))
                .bg(Color::Black),
        )
        .alignment(Alignment::Center),
        two,
    );
    frame.render_widget(
        Paragraph::new(vec![Line::from(format!("Punc: {}", settings.punctuation))])
            .block(
                Block::new()
                    .padding(Padding::new(0, 0, smart_area.height / 2, 0))
                    .bg(Color::Black),
            )
            .alignment(Alignment::Center),
        three,
    );
    frame.render_widget(
        Paragraph::new(vec![Line::from(format!("Zen: {}", settings.zen))])
            .block(
                Block::new()
                    .padding(Padding::new(0, 0, smart_area.height / 2, 0))
                    .bg(Color::Black),
            )
            .alignment(Alignment::Left),
        four,
    );
}
// todo, can probably break this out more
fn render_stats(frame: &mut Frame, app: &mut App, smart_area: Rect) {
    let [left, center, right] = {
        Layout::horizontal([
            Constraint::Fill(33),
            Constraint::Fill(33),
            Constraint::Fill(33),
        ])
        .areas(smart_area)
    };
    frame.render_widget(
        Paragraph::new(vec![Line::from(
            format!("WPM: {:.2}", app.previous_stats.wpm).as_str(),
        )])
        .block(
            Block::new()
                .padding(Padding::new(0, 0, smart_area.height / 2, 0))
                .bg(Color::Black),
        )
        .alignment(Alignment::Right),
        left,
    );
    frame.render_widget(
        Paragraph::new(vec![Line::from(
            format!("Accuracy: {:.2}", app.previous_stats.accuracy).as_str(),
        )])
        .block(
            Block::new()
                .padding(Padding::new(0, 0, smart_area.height / 2, 0))
                .bg(Color::Black),
        )
        .alignment(Alignment::Center),
        center,
    );
    frame.render_widget(
        Paragraph::new(vec![Line::from(
            format!("AWPM: {:.2}", app.previous_stats.awpm).as_str(),
        )])
        .block(
            Block::new()
                .padding(Padding::new(0, 0, smart_area.height / 2, 0))
                .bg(Color::Black),
        )
        .alignment(Alignment::Left),
        right,
    );
}

// Render black blocks for given area, useful for zen mode
fn render_blank(frame: &mut Frame, area: Rect) {
    frame.render_widget(Block::new().bg(Color::Black), area);
}

fn pause_popup(frame: &mut Frame, area: Rect) {
    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(
        Paragraph::new("PAUSED").alignment(Alignment::Center).block(
            Block::new()
                .bg(Color::Black)
                .padding(Padding::new(10, 10, area.height / 2, 0)),
        ),
        area,
    );
}
