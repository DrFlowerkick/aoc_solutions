use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, HighlightSpacing, List, ListItem, Paragraph, Wrap},
};

use super::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    // vertical layout
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(6),
        ])
        .split(frame.area());

    // the title
    let block = Block::bordered()
        .title("AOC 2019 Day 25 TUI")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let text = "This is the AOC 2019 Day 25 tui to manually solve day 25 IntCode challenge.";

    let title = Paragraph::new(text)
        .block(block)
        .fg(Color::Cyan)
        .bg(Color::Black)
        .centered();
    frame.render_widget(title, vertical[0]);

    // the footer
    let block = Block::bordered()
        .title("Navigation")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let mut text: Vec<Line> = Vec::new();
    if !app.error_message.is_empty() {
        text.push(Line::from(app.error_message.as_str()).bold().red());
    }
    text.push(Line::from(app.active_area.navigation_text()));
    text.push(Line::from(app.active_area.secondary_navigation_text()));
    text.push(Line::from("Press `Esc`, `Ctrl-C` or `q` to stop running."));

    let navigation = Paragraph::new(text)
        .block(block)
        .fg(Color::Cyan)
        .bg(Color::Black)
        .centered();
    frame.render_widget(navigation, vertical[2]);

    // layout of main area
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(vertical[1]);

    // ship room
    if let Some(ship_room) = app.ship_room.as_ref() {
        let block = Block::bordered()
            .title(ship_room.name.as_str())
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let mut text: Vec<Line> = Vec::new();
        let description = Line::from(ship_room.description.as_str())
            .bold()
            .left_aligned();
        text.push(description);
        text.push(Line::from(""));

        text.push(Line::from("Doors here lead:").italic().left_aligned());
        for door in ship_room.doors.iter() {
            let door = match door.as_str() {
                "north" => "▲ north",
                "east" => "► east",
                "south" => "▼ south",
                "west" => "◄ west",
                _ => unreachable!(),
            };
            text.push(Line::from(door).left_aligned());
        }
        let room = Paragraph::new(text)
            .block(block)
            .fg(app.fg_color_room())
            .bg(Color::Black)
            .wrap(Wrap { trim: true });
        frame.render_widget(room, main_layout[0]);

        // items of ship room
        let block = Block::bordered()
            .title("Items in Room")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let items: Vec<_> = ship_room
            .items
            .iter()
            .map(|i| ListItem::from("🪙 ".to_string() + i.as_str()))
            .collect();
        let items_of_room = List::new(items)
            .block(block)
            .fg(app.fg_color_items_of_room())
            .bg(Color::Black)
            .highlight_symbol(">")
            .highlight_style(Style::new().bold().fg(Color::Cyan))
            .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(items_of_room, main_layout[1], &mut app.state_items_of_room);
    } else {
        let block = Block::bordered()
            .title("== Entering Ship ==")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .fg(app.fg_color_room())
            .bg(Color::Black);
        frame.render_widget(block, main_layout[0]);

        let block = Block::bordered()
            .title("Items in Room")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .fg(app.fg_color_items_of_room())
            .bg(Color::Black);
        frame.render_widget(block, main_layout[1]);
    }

    // collected items
    if app.collected_items.is_empty() {
        let block = Block::bordered()
            .title("Collected Items")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .fg(app.fg_color_collected_items())
            .bg(Color::Black);
        frame.render_widget(block, main_layout[2]);
    } else {
        let block = Block::bordered()
            .title("Collected Items")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);
        let items: Vec<_> = app
            .collected_items
            .iter()
            .map(|i| ListItem::from("💰 ".to_string() + i.as_str()))
            .collect();
        let collected_items = List::new(items)
            .block(block)
            .fg(app.fg_color_collected_items())
            .bg(Color::Black)
            .highlight_symbol(">")
            .highlight_style(Style::new().bold().fg(Color::Cyan))
            .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(
            collected_items,
            main_layout[2],
            &mut app.state_collected_items,
        );
    }
}
