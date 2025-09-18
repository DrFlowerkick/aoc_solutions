// crawler to visit all rooms and collect all items

use chrono::Local;
use my_lib::my_algo_collection::collect_all_n_from_m_elements;
use petgraph::{
    Direction,
    algo::all_simple_paths,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};
use ratatui::widgets::{ListState, ScrollbarState};
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
    all_possible_item_combinations: Vec<Vec<String>>,
    current_combination: usize,
    removing_collected_items: bool,
}

impl RoomCrawler {
    pub fn toggle_status(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        collected_items: &[String],

        state_collected_items: &mut ListState,
        sleep: bool,
    ) {
        if !self.active
            && let Some(sh) = ship_room.as_ref()
        {
            self.add_action_message(&format!("Starting room crawler in room {}.", sh.name));
            self.ship_graph.clear();
            self.state = CrawlerState::Crawl;
            self.sleep = sleep;
            self.last_room = self.ship_graph.add_node(sh.name.clone());
            self.seen_rooms.insert(sh.name.clone(), self.last_room);
            self.active = true;
            self.collect_items(
                event_handler,
                ship_room,
                collected_items,
                state_collected_items,
            );
        } else {
            self.active = false;
        }
    }
    pub fn enter_room(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        collected_items: &[String],
        state_collected_items: &mut ListState,
    ) {
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
                    // check if new room is == Pressure-Sensitive Floor ==
                    if sh.name != "== Pressure-Sensitive Floor =="
                        || !sh.message.contains("ejected back to the checkpoint")
                    {
                        // collect items
                        self.collect_items(
                            event_handler,
                            ship_room,
                            collected_items,
                            state_collected_items,
                        );
                    }
                }
                CrawlerState::MoveToSecurityCheckpoint => {
                    self.add_action_message(&format!("Entered room {}", sh.name));
                    self.move_to_other_room(
                        event_handler,
                        ship_room,
                        collected_items,
                        state_collected_items,
                    );
                }
                CrawlerState::OpenSecurityDoor => {
                    if sh.name == "== Pressure-Sensitive Floor =="
                        && !sh.message.contains("ejected back to the checkpoint")
                    {
                        self.add_action_message(
                            "== Pressure-Sensitive Floor == accepted bot weight",
                        );
                        self.add_action_message(&sh.message);
                        self.add_action_message("Crawler finished.");
                        self.active = false;
                    } else if sh.name == "== Security Checkpoint ==" {
                        self.add_action_message("== Pressure-Sensitive Floor == ejected robot.");
                        self.prepare_next_combination(
                            event_handler,
                            ship_room,
                            collected_items,
                            state_collected_items,
                        );
                    }
                }
            }
        }
    }
    pub fn collect_items(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        collected_items: &[String],
        state_collected_items: &mut ListState,
    ) {
        if self.active
            && let Some(sh) = ship_room.as_ref()
        {
            match self.state {
                CrawlerState::Crawl => {
                    // handle bad items by ignoring them
                    // use this to debug a bad item
                    let bad_items = [];
                    // picking up all of these items result in either an unexpected halt of int computer
                    // or in case of "infinite loop" in an infinite loop
                    // or in case of "giant electromagnet" prevention of moving around
                    let ignore_bad_items = [
                        "escape pod",
                        "infinite loop",
                        "giant electromagnet",
                        "molten lava",
                        "photons",
                    ];
                    if sh.items.is_empty()
                        || sh
                            .items
                            .iter()
                            .any(|i| ignore_bad_items.contains(&i.as_str()))
                    {
                        self.move_to_other_room(
                            event_handler,
                            ship_room,
                            collected_items,
                            state_collected_items,
                        );
                    } else if sh.items.iter().any(|i| bad_items.contains(&i.as_str())) {
                        self.add_action_message("Debug item");
                        self.active = false;
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
                    if self.removing_collected_items {
                        self.set_current_combination(
                            event_handler,
                            ship_room,
                            collected_items,
                            state_collected_items,
                        );
                    } else {
                        self.prepare_next_combination(
                            event_handler,
                            ship_room,
                            collected_items,
                            state_collected_items,
                        );
                    }
                }
            }
        }
    }
    pub fn move_to_other_room(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        collected_items: &[String],
        state_collected_items: &mut ListState,
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
                            **d != ed
                        } else {
                            true
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
                        self.get_path_to_security_checkpoint(
                            event_handler,
                            ship_room,
                            collected_items,
                            state_collected_items,
                        );
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
                        event_handler.send(door_event(door_to_next_room));
                        self.last_room = next_room;
                        self.door = door_to_next_room.to_owned();
                        self.add_action_message(&format!(
                            "Moving from {} through door {}",
                            sh.name, self.door
                        ));
                        self.sleep();
                    } else {
                        self.get_all_possible_item_combinations(
                            event_handler,
                            ship_room,
                            collected_items,
                            state_collected_items,
                        );
                        self.state = CrawlerState::OpenSecurityDoor;
                        self.add_action_message("starting to try opening security door");
                    }
                }
                CrawlerState::OpenSecurityDoor => {
                    assert_eq!(sh.name, "== Security Checkpoint ==");
                    event_handler.send(AppEvent::Up);
                    self.door = "north".into();
                    self.add_action_message(&format!(
                        "Trying to move from {} through security door {}",
                        sh.name, self.door
                    ));
                    self.sleep();
                }
            }
        }
    }
    pub fn collect_message(
        &mut self,
        message: &str,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        collected_items: &[String],
        state_collected_items: &mut ListState,
    ) {
        if self.active {
            self.messages
                .push_front((Local::now().to_rfc2822(), message.to_string()));
            self.collect_items(
                event_handler,
                ship_room,
                collected_items,
                state_collected_items,
            );
        }
    }
    fn sleep(&self) {
        let sleep_duration = match self.state {
            CrawlerState::Crawl => Duration::from_millis(10),
            CrawlerState::MoveToSecurityCheckpoint => Duration::from_millis(10),
            CrawlerState::OpenSecurityDoor => Duration::from_millis(10),
        };
        if self.sleep {
            thread::sleep(sleep_duration);
        }
    }
    fn add_action_message(&mut self, message: &str) {
        self.messages
            .push_front((Local::now().to_rfc2822(), message.to_string()));
        while self.messages.len() > 100 {
            self.messages.pop_back();
        }
    }
    fn get_path_to_security_checkpoint(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        collected_items: &[String],
        state_collected_items: &mut ListState,
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
        self.move_to_other_room(
            event_handler,
            ship_room,
            collected_items,
            state_collected_items,
        );
    }
    fn get_all_possible_item_combinations(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        collected_items: &[String],
        state_collected_items: &mut ListState,
    ) {
        self.all_possible_item_combinations = (1..=collected_items.len())
            .flat_map(|n| collect_all_n_from_m_elements(collected_items, n).into_iter())
            .collect();
        assert!(ship_room.as_ref().unwrap().items.is_empty());

        self.set_current_combination(
            event_handler,
            ship_room,
            collected_items,
            state_collected_items,
        );
    }
    fn set_current_combination(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        collected_items: &[String],
        state_collected_items: &mut ListState,
    ) {
        if self.current_combination >= self.all_possible_item_combinations.len() {
            self.active = false;
            self.add_action_message(
                "Could not find combination of items, which opens security door.",
            );
        } else if let Some(position) = collected_items.iter().position(|i| {
            !self.all_possible_item_combinations[self.current_combination].contains(i)
        }) {
            state_collected_items.select(Some(position));
            event_handler.send(AppEvent::DropCollectedItem);
            self.add_action_message(&format!("Dropping item {}", collected_items[position]));
            self.sleep();
        } else {
            // try entering with this combination
            self.move_to_other_room(
                event_handler,
                ship_room,
                collected_items,
                state_collected_items,
            );
        }
    }
    fn prepare_next_combination(
        &mut self,
        event_handler: &mut EventHandler,
        ship_room: &Option<ShipRoom>,
        collected_items: &[String],
        state_collected_items: &mut ListState,
    ) {
        if ship_room.as_ref().unwrap().items.is_empty() {
            self.current_combination += 1;
            self.removing_collected_items = true;
            self.set_current_combination(
                event_handler,
                ship_room,
                collected_items,
                state_collected_items,
            );
        } else {
            self.removing_collected_items = false;
            event_handler.send(AppEvent::TakeRoomItem);
            self.add_action_message("Picking up all room items");
            self.sleep();
        }
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
