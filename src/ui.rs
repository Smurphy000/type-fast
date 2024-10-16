use futures::future::Join;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, HighlightSpacing, List, ListItem, Padding, Paragraph},
    Frame,
};
use tui_big_text::BigText;
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
        { Layout::vertical([Constraint::Fill(70), Constraint::Fill(30)]).areas(area) };

    match app.current_page {
        Pages::Menu => render_menu(frame, app, area, &mut a_buf),
        Pages::Typing => render_typing(frame, app, main_area),
        Pages::Stats => todo!(),
        Pages::Pause => render_typing(frame, app, main_area),
    }

    let mut buf: Buffer = Buffer::empty(smart_area);

    render_logging(frame, app, smart_area, &mut buf);
}

/// render logging information
/// this can be enabled via passing in settings
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

//  TODO center menu
fn render_menu(frame: &mut Frame, app: &mut App, smart_area: Rect, _buf: &mut Buffer) {
    // let [smart_area] = Layout::horizontal([Constraint::Fill(100)])
    //     .flex(ratatui::layout::Flex::Center)
    //     .areas(smart_area);
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
        .block(Block::new().padding(Padding::new(
            bottom_area.width / 4,
            bottom_area.width / 4,
            0,
            0,
        )));

    frame.render_widget(
        BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Full)
            .style(Style::new().white())
            .lines(vec!["Type Fast!".white().into()])
            .alignment(Alignment::Center)
            .build(),
        top_area,
    );
    frame.render_stateful_widget(list, bottom_area, &mut app.menu.current_selection);
}

fn render_typing(frame: &mut Frame, app: &mut App, smart_area: Rect) {
    let [top_area, bottom_area] =
        { Layout::vertical([Constraint::Fill(70), Constraint::Fill(30)]).areas(smart_area) };

    frame.render_widget(
        Paragraph::new(app.typing.text.clone())
            .block(
                Block::bordered()
                    .hidden()
                    .padding(Padding::new(0, 0, top_area.height / 2, 0))
                    .title_alignment(Alignment::Center), // .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Black).bg(Color::Black))
            .centered(),
        top_area,
    );

    // currently displaying typed text
    // let typing_string: String = app.typing.typing.iter().collect();
    // frame.render_widget(
    //     Span::raw(typing_string).style(Style::new().green()),
    //     bottom_area,
    // );

    // display the previous prompts stats
    let [left, center, right] = {
        Layout::horizontal([
            Constraint::Fill(33),
            Constraint::Fill(33),
            Constraint::Fill(33),
        ])
        .areas(bottom_area)
    };
    frame.render_widget(
        Paragraph::new(vec![Line::from(
            format!("WPM: {:.2}", app.previous_stats.wpm).as_str(),
        )])
        .block(Block::new().bg(Color::Black))
        .alignment(Alignment::Right),
        left,
    );
    frame.render_widget(
        Paragraph::new(vec![Line::from(
            format!("Accuracy: {:.2}", app.previous_stats.accuracy).as_str(),
        )])
        .block(Block::new().bg(Color::Black))
        .alignment(Alignment::Center),
        center,
    );
    frame.render_widget(
        Paragraph::new(vec![Line::from(
            format!("AWPM: {:.2}", app.previous_stats.awpm).as_str(),
        )])
        .block(Block::new().bg(Color::Black))
        .alignment(Alignment::Left),
        right,
    );

    if app.paused {
        pause_popup(frame, app, top_area);
    }
}

// TODO make pause take up the entire screen
fn pause_popup(frame: &mut Frame, app: &mut App, smart_area: Rect) {
    let list_items: Vec<ListItem> = app
        .pause_popup
        .options
        .iter()
        .map(|x| ListItem::from(x.to_string()).white())
        .collect();
    let list = List::new(list_items)
        .block(Block::bordered().fg(Color::Black).bg(Color::Black))
        .highlight_style(Style::default())
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    let area = popup_area(smart_area, 50, 50);
    frame.render_widget(Clear, area); //this clears out the background

    frame.render_stateful_widget(list, area, &mut app.pause_popup.current_selection);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
