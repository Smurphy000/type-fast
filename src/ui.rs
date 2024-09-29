use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, HighlightSpacing, List, ListItem, Paragraph, StatefulWidget},
    Frame,
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

use crate::app::{App, Pages};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    let area = frame.area();
    let mut a_buf: Buffer = Buffer::empty(area);
    let [main_area, smart_area] =
        { Layout::vertical([Constraint::Fill(50), Constraint::Fill(50)]).areas(area) };

    match app.current_page {
        Pages::Menu => render_menu(frame, app, area, &mut a_buf),
        Pages::Typing => {
            let text_string = String::from(app.current_words.join(" "));
            let text = Text::raw(text_string);
            frame.render_widget(
                Paragraph::new(text)
                    .block(
                        Block::bordered()
                            .title("Template")
                            .title_alignment(Alignment::Center)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                    .centered(),
                main_area,
            );
        }
        Pages::Stats => todo!(),
        Pages::Historical => todo!(),
    }

    let mut buf: Buffer = Buffer::empty(smart_area);

    render_logging(frame, app, smart_area, &mut buf);
}

fn render_logging(frame: &mut Frame, app: &App, smart_area: Rect, buf: &mut Buffer) {
    let logger = TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Magenta))
        .style_info(Style::default().fg(Color::Cyan))
        .output_separator(':')
        .output_timestamp(Some("%H:%M:%S".to_string()))
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_target(true)
        .output_file(true)
        .output_line(true)
        .block(Block::bordered().title("Logs"));

    frame.render_widget(logger, smart_area)
}

fn render_menu(frame: &mut Frame, app: &mut App, smart_area: Rect, buf: &mut Buffer) {
    let list_items: Vec<ListItem> = app
        .menu
        .options
        .iter()
        .map(|x| ListItem::from(x.to_string()))
        .collect();
    let list = List::new(list_items)
        .highlight_style(Style::default())
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, smart_area, &mut app.menu.current_selection);
}
