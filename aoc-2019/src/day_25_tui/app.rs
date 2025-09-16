use std::collections::HashSet;

use super::{AppEvent, Event, EventHandler, IntCodeHandler, ShipRoom, ui};
use color_eyre::eyre::bail;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    style::Color,
    widgets::ListState,
};

#[derive(Debug)]
pub enum ActiveArea {
    Room,
    ItemsOfRoom,
    CollectedItems,
}

impl ActiveArea {
    pub fn left(&self) -> Self {
        match self {
            ActiveArea::Room => ActiveArea::CollectedItems,
            ActiveArea::ItemsOfRoom => ActiveArea::Room,
            ActiveArea::CollectedItems => ActiveArea::ItemsOfRoom,
        }
    }
    pub fn right(&self) -> Self {
        match self {
            ActiveArea::Room => ActiveArea::ItemsOfRoom,
            ActiveArea::ItemsOfRoom => ActiveArea::CollectedItems,
            ActiveArea::CollectedItems => ActiveArea::Room,
        }
    }
    pub fn navigation_text(&self) -> &str {
        match self {
            ActiveArea::Room => {
                "Try door west with ◄ / Left, north with ▲ / Up, south with ▼ / Down east with ► / Right."
            }
            ActiveArea::ItemsOfRoom => {
                "Select previous item with ▲ / Up, next item with ▼ / Down, take up item with ► / Right."
            }
            ActiveArea::CollectedItems => {
                "Select previous item with ▲ / Up, next item with ▼ / Down, drop item to room with ◄ / Left."
            }
        }
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// int code handler
    pub int_code_handler: IntCodeHandler,
    /// active area
    pub active_area: ActiveArea,
    /// last received text message
    pub last_text_message: String,
    /// current ship room
    pub ship_room: Option<ShipRoom>,
    /// visited rooms
    pub visited_rooms: HashSet<String>,
    /// list state for items of room
    pub state_items_of_room: ListState,
    /// collected items
    pub collected_items: Vec<String>,
    /// list state for collected items
    pub state_collected_items: ListState,
    /// flag for check of inventory
    pub flag_check_inventory: bool,
}

impl Default for App {
    fn default() -> Self {
        let (int_code_handler, int_code_task) = IntCodeHandler::new();
        Self {
            running: true,
            events: EventHandler::new(int_code_task),
            int_code_handler,
            active_area: ActiveArea::Room,
            last_text_message: String::new(),
            ship_room: None,
            visited_rooms: HashSet::new(),
            state_items_of_room: ListState::default(),
            collected_items: Vec::new(),
            state_collected_items: ListState::default(),
            flag_check_inventory: false,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| ui(frame, &mut self))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    /// Run the application's main loop in silent mode (no tui!)
    pub async fn run_no_tui(mut self) -> color_eyre::Result<()> {
        while self.running {
            self.handle_events().await?;
        }
        Ok(())
    }

    /// Handles events
    pub async fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next().await? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => {
                if let crossterm::event::Event::Key(key_event) = event {
                    self.handle_key_events(key_event)?;
                }
            }
            Event::App(app_event) => match app_event {
                AppEvent::ShipRoom(ship_room) => {
                    if ship_room.items.is_empty() {
                        self.state_items_of_room.select(None);
                    } else {
                        self.state_items_of_room.select_first();
                    }
                    self.visited_rooms.insert(ship_room.name.clone());
                    self.ship_room = Some(ship_room);
                    self.last_text_message.clear();
                }
                AppEvent::TextMessage(text) => {
                    self.last_text_message = text;
                    if self.flag_check_inventory {
                        self.check_inventory()?;
                    }
                }
                AppEvent::Up => {
                    self.int_code_handler.move_up()?;
                }
                AppEvent::Down => {
                    self.int_code_handler.move_down()?;
                }
                AppEvent::Right => {
                    self.int_code_handler.move_right()?;
                }
                AppEvent::Left => {
                    self.int_code_handler.move_left()?;
                }
                AppEvent::PreviousRoomItem => {
                    self.state_items_of_room.select_previous();
                }
                AppEvent::NexRoomItem => {
                    self.state_items_of_room.select_next();
                }
                AppEvent::PreviousCollectedItem => {
                    self.state_collected_items.select_previous();
                }
                AppEvent::NextCollectedItem => {
                    self.state_collected_items.select_next();
                }
                AppEvent::TakeRoomItem => {
                    self.take_room_item()?;
                }
                AppEvent::DropCollectedItem => {
                    self.drop_collected_item()?;
                }
                AppEvent::CheckInventory => {
                    self.int_code_handler.send_inventory_request()?;
                }
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Up => match self.active_area {
                ActiveArea::Room => self.events.send(AppEvent::Up),
                ActiveArea::ItemsOfRoom => self.events.send(AppEvent::PreviousRoomItem),
                ActiveArea::CollectedItems => self.events.send(AppEvent::PreviousCollectedItem),
            },
            KeyCode::Down => match self.active_area {
                ActiveArea::Room => self.events.send(AppEvent::Down),
                ActiveArea::ItemsOfRoom => self.events.send(AppEvent::NexRoomItem),
                ActiveArea::CollectedItems => self.events.send(AppEvent::NextCollectedItem),
            },
            KeyCode::Right => match self.active_area {
                ActiveArea::Room => self.events.send(AppEvent::Right),
                ActiveArea::ItemsOfRoom => self.events.send(AppEvent::TakeRoomItem),
                ActiveArea::CollectedItems => {}
            },
            KeyCode::Left => match self.active_area {
                ActiveArea::Room => self.events.send(AppEvent::Left),
                ActiveArea::ItemsOfRoom => {}
                ActiveArea::CollectedItems => self.events.send(AppEvent::DropCollectedItem),
            },
            KeyCode::Home => self.active_area = self.active_area.left(),
            KeyCode::End => self.active_area = self.active_area.right(),
            KeyCode::Char('i') => self.events.send(AppEvent::CheckInventory),
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// returns fg color of main area
    pub fn fg_color_room(&self) -> Color {
        match self.active_area {
            ActiveArea::Room => Color::Yellow,
            _ => Color::Cyan,
        }
    }

    pub fn fg_color_items_of_room(&self) -> Color {
        match self.active_area {
            ActiveArea::ItemsOfRoom => Color::Yellow,
            _ => Color::Cyan,
        }
    }

    pub fn fg_color_collected_items(&self) -> Color {
        match self.active_area {
            ActiveArea::CollectedItems => Color::Yellow,
            _ => Color::Cyan,
        }
    }

    /// handle items
    pub fn take_room_item(&mut self) -> color_eyre::Result<()> {
        if let Some(selected) = self.state_items_of_room.selected()
            && self.ship_room.is_some()
            && selected < self.ship_room.as_ref().unwrap().items.len()
        {
            self.state_items_of_room.select_previous();
            let room_item = self.ship_room.as_mut().unwrap().items.remove(selected);
            self.int_code_handler.take_room_item(&room_item)?;
            let selected_collected_item = self.collected_items.len();
            self.collected_items.push(room_item);
            self.state_collected_items
                .select(Some(selected_collected_item));
        }
        Ok(())
    }

    pub fn drop_collected_item(&mut self) -> color_eyre::Result<()> {
        if let Some(selected) = self.state_collected_items.selected()
            && self.ship_room.is_some()
            && selected < self.collected_items.len()
        {
            self.state_collected_items.select_previous();
            let collected_item = self.collected_items.remove(selected);
            self.int_code_handler.drop_collected_item(&collected_item)?;
            let selected_room_item = self.ship_room.as_ref().unwrap().items.len();
            self.ship_room.as_mut().unwrap().items.push(collected_item);
            self.state_items_of_room.select(Some(selected_room_item));
        }
        Ok(())
    }

    pub fn check_inventory(&mut self) -> color_eyre::Result<()> {
        let err = if let Some(inventory) = self
            .last_text_message
            .strip_prefix("Items in your inventory:\n")
        {
            let inventory: HashSet<_> = inventory
                .lines()
                .filter_map(|i| i.strip_prefix("- "))
                .collect();
            let collected_items: HashSet<_> =
                self.collected_items.iter().map(|ci| ci.as_str()).collect();
            inventory != collected_items
        } else {
            // empty inventory -> err if we have collected items
            !self.collected_items.is_empty()
        };
        if err {
            bail!("inventory is not equal to collected items.");
        }
        self.flag_check_inventory = false;
        Ok(())
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
