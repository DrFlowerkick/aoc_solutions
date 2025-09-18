// int code handling in day 25

use super::{AppEvent, Event};
use crate::days::day_05::{IntCodeComputer, IntOut};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender, error::TryRecvError};

#[derive(Debug)]
pub struct IntCodeHandler {
    in_sender: UnboundedSender<i64>,
}

impl IntCodeHandler {
    pub fn new() -> (Self, IntCodeTask) {
        let (in_sender, in_receiver) = mpsc::unbounded_channel();
        let (out_sender, out_receiver) = mpsc::unbounded_channel();
        let err_sender = out_sender.clone();

        thread::spawn(move || {
            let input = include_str!("../../../../aoc_input/aoc-2019/day_25.txt");
            let mut code = IntCodeComputer::from(input);
            if let Err(err) =
                code.run_int_code_with_mpsc(in_receiver, out_sender, None, Duration::from_millis(1))
            {
                let _ = err_sender.send(Err(err));
            }
        });
        (Self { in_sender }, IntCodeTask { out_receiver })
    }
    pub fn move_up(&self) -> color_eyre::Result<()> {
        for ch in "north\n".chars() {
            self.in_sender.send(ch as i64)?;
        }
        Ok(())
    }
    pub fn move_down(&self) -> color_eyre::Result<()> {
        for ch in "south\n".chars() {
            self.in_sender.send(ch as i64)?;
        }
        Ok(())
    }
    pub fn move_right(&self) -> color_eyre::Result<()> {
        for ch in "east\n".chars() {
            self.in_sender.send(ch as i64)?;
        }
        Ok(())
    }
    pub fn move_left(&self) -> color_eyre::Result<()> {
        for ch in "west\n".chars() {
            self.in_sender.send(ch as i64)?;
        }
        Ok(())
    }
    pub fn take_room_item(&self, item: &str) -> color_eyre::Result<()> {
        for ch in "take ".chars() {
            self.in_sender.send(ch as i64)?;
        }
        for ch in item.chars() {
            self.in_sender.send(ch as i64)?;
        }
        self.in_sender.send('\n' as i64)?;
        Ok(())
    }
    pub fn drop_collected_item(&self, item: &str) -> color_eyre::Result<()> {
        for ch in "drop ".chars() {
            self.in_sender.send(ch as i64)?;
        }
        for ch in item.chars() {
            self.in_sender.send(ch as i64)?;
        }
        self.in_sender.send('\n' as i64)?;
        Ok(())
    }
    pub fn send_inventory_request(&mut self) -> color_eyre::Result<()> {
        for ch in "inv\n".chars() {
            self.in_sender.send(ch as i64)?;
        }
        Ok(())
    }
}

pub struct IntCodeTask {
    out_receiver: UnboundedReceiver<Result<(i64, IntOut), String>>,
}

impl IntCodeTask {
    pub async fn receive_messages(
        &mut self,
        sender: mpsc::UnboundedSender<Event>,
    ) -> color_eyre::Result<()> {
        let mut message = String::new();
        loop {
            match self.out_receiver.try_recv() {
                Ok(Ok((_id, IntOut::Out(value)))) => {
                    if value > 255 {
                        let _ = sender.send(Event::App(AppEvent::NoneAscii(value)));
                    } else {
                        let ch = (value as u8) as char;
                        message.push(ch);
                        if message.ends_with("Command?") {
                            // send raw string
                            let _ = sender.send(Event::App(AppEvent::RawMessage(message.clone())));
                            // handle == Pressure-Sensitive Floor ==
                            if message.trim().starts_with("== Pressure-Sensitive Floor ==")
                                && let Some(pos) = message.find("== Security Checkpoint ==")
                            {
                                // robot did not pass == Pressure-Sensitive Floor ==
                                // and was ejected back to == Security Checkpoint ==
                                let (ps, sc) = message.split_at(pos);
                                let _ = sender.send(Event::App(AppEvent::ShipRoom(
                                    ShipRoom::try_from(ps).unwrap(),
                                )));
                                message = sc.to_string();
                            }
                            match ShipRoom::try_from(message.as_str()) {
                                Ok(ship_room) => {
                                    let _ = sender.send(Event::App(AppEvent::ShipRoom(ship_room)));
                                }
                                Err(err) => {
                                    let _ = sender.send(Event::App(AppEvent::TextMessage(err)));
                                }
                            }
                            message.clear();
                        }
                    }
                }
                Ok(Ok((_, IntOut::Halt))) => {
                    if message.trim().ends_with("main airlock.\"") {
                        // reached end
                        let _ = sender.send(Event::App(AppEvent::ShipRoom(
                            ShipRoom::try_from(message.as_str()).unwrap(),
                        )));
                        let code: String = message.chars().filter(|c| c.is_ascii_digit()).collect();
                        let _ = sender.send(Event::App(AppEvent::TextMessage(format!(
                            "Code for main airlock: {code}\nGame Ends here\nHappy Christmas🎄🎁🎁🎁"
                        ))));
                    } else if !message.is_empty() {
                        let _ = sender.send(Event::App(AppEvent::TextMessage(format!(
                            "{}\nGame ends. You lost.",
                            message.trim()
                        ))));
                    } else {
                        let _ = sender.send(Event::App(AppEvent::TextMessage(
                            "unexpected halt of Int Computer.".into(),
                        )));
                    }
                    let _ = sender.send(Event::App(AppEvent::IntCodeHalt));
                }
                Ok(Ok((_, IntOut::None))) => unreachable!(),
                Ok(Err(err)) => {
                    let _ = sender.send(Event::App(AppEvent::TextMessage(format!(
                        "Int Computer returned error:\n{err}\nGame ends. You lost."
                    ))));
                }
                Err(TryRecvError::Empty) => {
                    thread::sleep(Duration::from_millis(1));
                }
                Err(TryRecvError::Disconnected) => return Ok(()),
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ShipRoom {
    pub name: String,
    pub description: String,
    pub doors: Vec<String>,
    pub items: Vec<String>,
    pub message: String,
}

impl TryFrom<&str> for ShipRoom {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().starts_with("==") {
            let mut ship_room = ShipRoom::default();
            for block in value.trim().split("\n\n") {
                if block.starts_with("==") {
                    let (name, description) = block.split_once('\n').unwrap();
                    ship_room.name = name.into();
                    ship_room.description = description.into();
                } else if let Some(doors) = block.strip_prefix("Doors here lead:\n") {
                    ship_room.doors = doors
                        .lines()
                        .filter_map(|d| d.strip_prefix("- "))
                        .map(|d| d.to_string())
                        .collect();
                } else if let Some(items) = block.strip_prefix("Items here:\n") {
                    ship_room.items = items
                        .lines()
                        .filter_map(|d| d.strip_prefix("- "))
                        .map(|d| d.to_string())
                        .collect();
                } else if block != "Command?" {
                    ship_room.message = block.to_string();
                }
            }
            Ok(ship_room)
        } else {
            let no_ship_room_message = value.trim().strip_suffix("Command?").unwrap().trim();
            Err(no_ship_room_message.to_string())
        }
    }
}

impl ShipRoom {
    pub fn get_name(&self) -> Option<&str> {
        self.name.strip_prefix("==")?.strip_suffix("==")
    }
}
