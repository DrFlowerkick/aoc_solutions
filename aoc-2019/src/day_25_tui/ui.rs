use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols::scrollbar,
    text::Line,
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, Paragraph, Scrollbar,
        ScrollbarOrientation, Wrap,
    },
};

use super::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    // vertical layout
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(5),
            Constraint::Percentage(43),
            Constraint::Percentage(43),
            Constraint::Percentage(9),
        ])
        .split(frame.area());

    // the title
    let block = Block::bordered()
        .title(" AOC 2019 Day 25 TUI ")
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
        .title(" Navigation ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let first_line = if app.room_crawler.active {
        "Room crawler is active. Deactivate to switch active area."
    } else {
        "Switch active area with Pos1/Home and End/Ende."
    };

    let text = vec![
        Line::from(first_line),
        Line::from(app.active_area.navigation_text(app.room_crawler.active)),
        Line::from("Compare collected items with inventory from int code with: i."),
        Line::from("Press `Esc`, `Ctrl-C` or `q` to stop running."),
    ];

    let navigation = Paragraph::new(text)
        .block(block)
        .fg(Color::Cyan)
        .bg(Color::Black)
        .centered();
    frame.render_widget(navigation, vertical[3]);

    // layout of main area
    let main_layout_top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(vertical[1]);
    let area_items_in_room = main_layout_top[1];
    let area_collected_items = main_layout_top[2];
    let area_visited_rooms = main_layout_top[3];

    let left_main_layout_top = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout_top[0]);
    let area_room = left_main_layout_top[0];
    let area_last_message = left_main_layout_top[1];

    let main_layout_bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(vertical[2]);
    let area_raw_message = main_layout_bottom[0];
    let area_crawler_messages = main_layout_bottom[1];

    // ship room
    if let Some(ship_room) = app.ship_room.as_ref() {
        let block = Block::bordered()
            .title(ship_room.get_name().unwrap())
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
        text.push(Line::from(""));
        let message = Line::from(ship_room.message.as_str()).bold().left_aligned();
        text.push(message);

        let room = Paragraph::new(text)
            .block(block)
            .fg(app.fg_color_room())
            .bg(Color::Black)
            .wrap(Wrap { trim: true });
        frame.render_widget(room, area_room);

        // items of ship room
        let block = Block::bordered()
            .title(" Items in Room ")
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
        frame.render_stateful_widget(
            items_of_room,
            area_items_in_room,
            &mut app.state_items_of_room,
        );
    } else {
        let block = Block::bordered()
            .title(" Entering Ship ")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .fg(app.fg_color_room())
            .bg(Color::Black);
        frame.render_widget(block, area_room);

        let block = Block::bordered()
            .title(" Items in Room ")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .fg(app.fg_color_items_of_room())
            .bg(Color::Black);
        frame.render_widget(block, area_items_in_room);
    }

    // last text message
    let block = Block::bordered()
        .title(" Last message from robot ")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    let message = Paragraph::new(app.last_text_message.as_str())
        .block(block)
        .left_aligned()
        .fg(Color::Cyan)
        .bg(Color::Black)
        .wrap(Wrap { trim: true });
    frame.render_widget(message, area_last_message);

    // last raw message
    let block = Block::bordered()
        .title(" Last raw message from robot ")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    let raw_message = Paragraph::new(app.last_raw_message.as_str().trim())
        .block(block)
        .left_aligned()
        .fg(Color::Cyan)
        .bg(Color::Black)
        .wrap(Wrap { trim: true });
    frame.render_widget(raw_message, area_raw_message);

    // visited rooms
    let block = Block::bordered()
        .title(" Visited rooms ")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    let mut rooms: Vec<_> = app.visited_rooms.iter().collect();
    rooms.sort();
    let rooms: Vec<_> = rooms
        .iter()
        .map(|r| Line::from(r.as_str()).left_aligned())
        .collect();

    let visited_rooms = Paragraph::new(rooms)
        .block(block)
        .left_aligned()
        .fg(Color::Cyan)
        .bg(Color::Black)
        .wrap(Wrap { trim: true });
    frame.render_widget(visited_rooms, area_visited_rooms);

    // collected items
    if app.collected_items.is_empty() {
        let block = Block::bordered()
            .title(" Collected Items ")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .fg(app.fg_color_collected_items())
            .bg(Color::Black);
        frame.render_widget(block, area_collected_items);
    } else {
        let block = Block::bordered()
            .title(" Collected Items ")
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
            area_collected_items,
            &mut app.state_collected_items,
        );
    }

    // crawler messages
    if app.room_crawler.messages.is_empty() {
        let block = Block::bordered()
            .title(" Crawler messages ")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .fg(app.fg_color_crawler_messages())
            .bg(Color::Black);
        frame.render_widget(block, area_crawler_messages);
    } else {
        let block = Block::bordered()
            .title(" Crawler messages ")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);
        let messages: Vec<_> = app
            .room_crawler
            .messages
            .iter()
            .map(|(i, m)| format!("{}: {}", i, m))
            .collect();

        let messages: Vec<_> = messages
            .iter()
            .map(|r| Line::from(r.as_str()).left_aligned())
            .collect();
        app.room_crawler.scroll_state =
            app.room_crawler.scroll_state.content_length(messages.len());
        let crawler_messages = Paragraph::new(messages)
            .block(block)
            .left_aligned()
            .fg(app.fg_color_crawler_messages())
            .bg(Color::Black)
            .wrap(Wrap { trim: true })
            .scroll((app.room_crawler.scroll as u16, 0));
        frame.render_widget(crawler_messages, area_crawler_messages);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight).symbols(scrollbar::VERTICAL),
            area_crawler_messages,
            &mut app.room_crawler.scroll_state,
        );
    }
}
