use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap, Tabs},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    // Create Initial Layout of Main
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.area());

    // title

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "This is a cool application, but its ugly!",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);


    // Display a list of stuff and things
    // Data loaded in main.rs

    let mut list_items = Vec::<ListItem>::new();

    for key in app.tasks_map.keys() {
        list_items.push(ListItem::new(
            Line::from(Span::styled(
                format!("{: <25}: {}", key, app.tasks_map.get(key).unwrap()),
                Style::default().fg(Color::Yellow),
            ))
        ));
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);

    // Footer stuff and things

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::TicketDashboard => {
                Span::styled("Should not be here!", Style::default().fg(Color::Red))
            }
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled(" Some stuff will go here ", Style::default().fg(Color::White)),
    ];

    let screen_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));


    let current_key_hints = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::TicketDashboard => Span::styled(
                "(q) to quit",
                Style::default().fg(Color::Red),
            )
        }
    };

    let key_notes_footer = Paragraph::new(
        Line::from(current_key_hints)
    ).block(Block::default().borders(Borders::ALL));

    let footer_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[2]);

    frame.render_widget(screen_footer, footer_chunk[0]);
    frame.render_widget(key_notes_footer, footer_chunk[1]);

    let ascii_art = "
        ──────▄▀▄─────▄▀▄
    ─────▄█░░▀▀▀▀▀░░█▄
    ─▄▄──█░░░░░░░░░░░█──▄▄
    █▄▄█─█░░▀░░┬░░▀░░█─█▄▄█
    ";
    // Cat pop out!
    if app.display_cat {
        let popup_block = Block::default()
            .title("This is a cat")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));

        let area = centered_rect(10, 10, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let cat_widget = Paragraph::new(ascii_art.clone()).block(Block::default().borders(Borders::NONE));

        frame.render_widget(cat_widget, popup_chunk[0]);
    }

}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
