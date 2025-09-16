// crawler to visit all rooms and collect all items

use chrono::Local;
use petgraph::{
    Direction,
    algo::all_simple_paths,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};
use ratatui::widgets::ScrollbarState;
use std::{
    collections::{HashMap, VecDeque, hash_map::RandomState},
    thread,
    time::Duration,
};

use super::{AppEvent, EventHandler, ShipRoom};

#[derive(Debug, Default)]
enum CrawlerState {
    #[default]
    Crawl,
    MoveToSecurityCheckpoint,
    OpenSecurityDoor,
}

#[derive(Debug, Default)]
pub struct RoomCrawler {
    pub active: bool,
    pub messages: VecDeque<(String, String)>,
    pub scroll_state: ScrollbarState,
    pub scroll: usize,
    state: CrawlerState,
    ship_graph: DiGraph<String, String>,
    last_room: NodeIndex,
    door: String,
    seen_rooms: HashMap<String, NodeIndex>,
    sleep: bool,
    path_to_sc: VecDeque<NodeIndex>,
}

impl RoomCrawler {
    pub fn toggle_status(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        sleep: bool,
    ) {
        if !self.active
            && let Some(sh) = ship_room.as_ref()
        {
            self.add_action_message(&format!("Starting room crawler in room {}.", sh.name));
            self.ship_graph.clear();
            self.sleep = sleep;
            self.last_room = self.ship_graph.add_node(sh.name.clone());
            self.seen_rooms.insert(sh.name.clone(), self.last_room);
            self.active = true;
            self.collect_items(event_handler, ship_room);
        } else {
            self.active = false;
        }
    }
    pub fn enter_room(&mut self, event_handler: &mut EventHandler, ship_room: &Option<ShipRoom>) {
        if self.active
            && let Some(sh) = ship_room.as_ref()
        {
            match self.state {
                CrawlerState::Crawl => {
                    if let Some(last_room) = self.ship_graph.node_weight(self.last_room)
                        && last_room != &sh.name
                    {
                        if let Some(visited_room) = self.seen_rooms.get(&sh.name) {
                            // entered visited room: going backwards or entered same room twice through another door?
                            if !self
                                .ship_graph
                                .edges_directed(self.last_room, Direction::Incoming)
                                .any(|e| e.source() == *visited_room)
                            {
                                // entered same room twice through another door
                                let (event, door) = reverse_door(&self.door);
                                // move back to other room
                                event_handler.send(event);
                                self.last_room = *visited_room;
                                self.door = door.to_string();
                                self.add_action_message(&format!(
                                    "Moving back from already visited room {}",
                                    sh.name
                                ));
                                self.sleep();
                                return;
                            } else {
                                self.add_action_message(&format!("Entered room {}", sh.name));
                            }
                        } else {
                            // entered new room
                            self.add_action_message(&format!("Entered new room {}", sh.name));
                            let node = self.ship_graph.add_node(sh.name.clone());
                            self.seen_rooms.insert(sh.name.clone(), node);
                            self.ship_graph
                                .add_edge(self.last_room, node, self.door.clone());
                        }
                    }
                    // collect items
                    self.collect_items(event_handler, ship_room);
                }
                CrawlerState::MoveToSecurityCheckpoint => {
                    self.add_action_message(&format!("Entered room {}", sh.name));
                    self.move_to_other_room(event_handler, ship_room);
                }
                CrawlerState::OpenSecurityDoor => {
                    todo!()
                }
            }
        }
    }
    pub fn collect_items(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
    ) {
        if self.active
            && let Some(sh) = ship_room.as_ref()
        {
            match self.state {
                CrawlerState::Crawl => {
                    // handle bad items by ignoring them
                    let bad_items = [
                        "giant electromagnet",
                        "photons",
                        "molten lava",
                        "infinite loop",
                        "escape pod",
                    ];
                    if sh.items.is_empty()
                        || sh.items.iter().any(|i| bad_items.contains(&i.as_str()))
                    {
                        self.move_to_other_room(event_handler, ship_room);
                    } else {
                        event_handler.send(AppEvent::TakeRoomItem);
                        self.add_action_message("Tacking an item.");
                        self.sleep();
                    }
                }
                CrawlerState::MoveToSecurityCheckpoint => {
                    unreachable!()
                }
                CrawlerState::OpenSecurityDoor => {
                    todo!()
                }
            }
        }
    }
    pub fn move_to_other_room(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
    ) {
        if self.active
            && let Some(sh) = ship_room.as_ref()
        {
            match self.state {
                CrawlerState::Crawl => {
                    self.last_room = *self.seen_rooms.get(&sh.name).unwrap();
                    let entry_door = self
                        .ship_graph
                        .edges_directed(self.last_room, Direction::Incoming)
                        .map(|e| e.weight())
                        .next()
                        .map(|d| reverse_door(d));
                    let remaining_doors: Vec<_> = sh
                        .doors
                        .iter()
                        .filter(|d| {
                            !self
                                .ship_graph
                                .edges_directed(self.last_room, Direction::Outgoing)
                                .any(|e| e.weight() == *d)
                        })
                        .map(|d| d.as_str())
                        .collect();
                    if let Some(next_door) = remaining_doors.iter().find(|d| {
                        if let Some((_, ed)) = entry_door {
                            **d != ed && (**d != "north" || sh.name != "== Security Checkpoint ==")
                        } else {
                            **d != "north" || sh.name != "== Security Checkpoint =="
                        }
                    }) {
                        // move through next door
                        self.door = next_door.to_string();
                        let event = door_event(next_door);
                        event_handler.send(event);
                        self.add_action_message(&format!(
                            "Moving from {} through door {}.",
                            sh.name, self.door
                        ));
                        self.sleep();
                    } else if let Some((event, door)) = entry_door {
                        // move backward through entry door
                        self.door = door.to_string();
                        event_handler.send(event);
                        self.add_action_message(&format!(
                            "Moving back from {} through entry door {}.",
                            sh.name, self.door
                        ));
                        self.sleep();
                    } else {
                        // no more door left -> crawler finished
                        assert!(remaining_doors.is_empty());
                        self.add_action_message(
                            "Crawler visited all rooms and collected all save items",
                        );
                        self.get_path_to_security_checkpoint(event_handler, ship_room);
                    }
                }
                CrawlerState::MoveToSecurityCheckpoint => {
                    if let Some(next_room) = self.path_to_sc.pop_front() {
                        let door_to_next_room = self
                            .ship_graph
                            .edges_directed(self.last_room, Direction::Outgoing)
                            .find(|e| e.target() == next_room)
                            .map(|e| e.weight())
                            .unwrap();
                        event_handler.send(door_event(&door_to_next_room));
                        self.last_room = next_room;
                        self.door = door_to_next_room.to_owned();
                        self.add_action_message(&format!(
                            "Moving from {} through door {}",
                            sh.name, self.door
                        ));
                        self.sleep();
                    } else {
                        self.add_action_message("start to try opening security door");
                        self.state = CrawlerState::OpenSecurityDoor;
                    }
                }
                CrawlerState::OpenSecurityDoor => {
                    todo!()
                }
            }
        }
    }
    pub fn collect_message(
        &mut self,
        message: &str,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
    ) {
        if self.active {
            self.messages
                .push_front((Local::now().to_rfc2822(), message.to_string()));
            self.collect_items(event_handler, ship_room);
        }
    }
    fn sleep(&self) {
        if self.sleep {
            thread::sleep(Duration::from_millis(100));
        }
    }
    fn add_action_message(&mut self, message: &str) {
        self.messages
            .push_front((Local::now().to_rfc2822(), message.to_string()));
    }
    fn get_path_to_security_checkpoint(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
    ) {
        let security_checkpoint_node = *self
            .seen_rooms
            .get(&String::from("== Security Checkpoint =="))
            .unwrap();
        let mut path_to_sc: VecDeque<_> = all_simple_paths::<VecDeque<_>, _, RandomState>(
            &self.ship_graph,
            self.last_room,
            security_checkpoint_node,
            0,
            None,
        )
        .collect();
        assert_eq!(path_to_sc.len(), 1);
        self.path_to_sc = path_to_sc.pop_front().unwrap();
        assert_eq!(self.path_to_sc.pop_front().unwrap(), self.last_room);
        self.state = CrawlerState::MoveToSecurityCheckpoint;
        self.add_action_message("Moving to Security Checkpoint");
        self.move_to_other_room(event_handler, ship_room);
    }
}

fn door_event(door: &str) -> AppEvent {
    match door {
        "north" => AppEvent::Up,
        "south" => AppEvent::Down,
        "east" => AppEvent::Right,
        "west" => AppEvent::Left,
        _ => unreachable!(),
    }
}

fn reverse_door(door: &str) -> (AppEvent, &str) {
    match door {
        "north" => (AppEvent::Down, "south"),
        "south" => (AppEvent::Up, "north"),
        "east" => (AppEvent::Left, "west"),
        "west" => (AppEvent::Right, "east"),
        _ => unreachable!(),
    }
}
