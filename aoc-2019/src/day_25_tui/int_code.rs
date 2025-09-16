// int code handling in day 25

use super::{AppEvent, Event};
use crate::days::day_05::IntCodeComputer;
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

        thread::spawn(move || {
            let input = include_str!("../../../../aoc_input/aoc-2019/day_25.txt");
            let mut code = IntCodeComputer::from(input);
            code.run_int_code_with_mpsc(in_receiver, out_sender, None, Duration::from_millis(1))
                .unwrap();
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
}

pub struct IntCodeTask {
    out_receiver: UnboundedReceiver<(i64, i64)>,
}

impl IntCodeTask {
    pub async fn receive_messages(
        &mut self,
        sender: mpsc::UnboundedSender<Event>,
    ) -> color_eyre::Result<()> {
        let mut message = String::new();
        loop {
            match self.out_receiver.try_recv() {
                Ok((_id, value)) => {
                    if value > 255 {
                        color_eyre::eyre::bail!("received none ascii char");
                    }
                    let ch = (value as u8) as char;
                    message.push(ch);
                    if message.ends_with("Command?") {
                        match ShipRoom::try_from(message.as_str()) {
                            Ok(ship_room) => {
                                let _ = sender.send(Event::App(AppEvent::ShipRoom(ship_room)));
                            }
                            Err(err) => {
                                let _ = sender.send(Event::App(AppEvent::ErrorMessage(err)));
                            }
                        }
                        message.clear();
                    }
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
                }
            }
            Ok(ship_room)
        } else {
            let err_message = value.trim().split('\n').next().unwrap();
            Err(err_message.to_string())
        }
    }
}
