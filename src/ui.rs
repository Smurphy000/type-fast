use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, HighlightSpacing, List, ListItem, Padding, Paragraph},
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
        Pages::Typing => render_typing(frame, app, main_area),
        Pages::Stats => todo!(),
        Pages::Historical => todo!(),
    }

    let mut buf: Buffer = Buffer::empty(smart_area);

    render_logging(frame, app, smart_area, &mut buf);
}

fn render_logging(frame: &mut Frame, _app: &App, smart_area: Rect, _buf: &mut Buffer) {
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

fn render_menu(frame: &mut Frame, app: &mut App, smart_area: Rect, _buf: &mut Buffer) {
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

fn render_typing(frame: &mut Frame, app: &mut App, smart_area: Rect) {
    let [top_area, bottom_area] =
        { Layout::vertical([Constraint::Fill(50), Constraint::Fill(50)]).areas(smart_area) };

    frame.render_widget(
        Paragraph::new(app.typing.text.clone())
            .block(
                Block::bordered()
                    .hidden()
                    .padding(Padding::new(0, 0, top_area.height / 2, 0))
                    .title_alignment(Alignment::Center), // .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        top_area,
    );

    // currently displaying typed text
    let typing_string: String = app.typing.typing.iter().collect();
    frame.render_widget(
        Span::raw(typing_string).style(Style::new().green()),
        bottom_area,
    );
}
