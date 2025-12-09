//!day_20.rs

use anyhow::{Result, anyhow};
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct FlipFlop {
    state: bool,
}

impl FlipFlop {
    fn recieve_pulse(&mut self, pulse: bool) -> Option<bool> {
        if pulse {
            return None;
        }
        self.state = !self.state;
        Some(self.state)
    }
}

#[derive(Default)]
struct Conjunction {
    memory: HashMap<String, bool>,
}

impl Conjunction {
    fn recieve_pulse(&mut self, pulse: bool, sender: &String) -> Result<bool> {
        let state = match self.memory.get_mut(sender) {
            Some(s) => s,
            None => return Err(anyhow!("internal error unknown sender")),
        };
        *state = pulse;
        Ok(!self.memory.values().all(|v| *v))
    }
}

#[derive(Default)]
struct Message {
    sender: String,
    reciever: String,
    pulse: bool,
}

impl Message {
    fn new_button_push() -> Self {
        Message {
            sender: "".into(),
            reciever: "broadcaster".into(),
            pulse: false,
        }
    }
    fn new_pulse(sender: String, reciever: String, pulse: bool) -> Self {
        Message {
            sender,
            reciever,
            pulse,
        }
    }
}

#[derive(Default)]
struct Server {
    queue: VecDeque<Message>,
    low_pulse_count: u64,
    high_pulse_count: u64,
    recievers: HashMap<String, Vec<String>>,
    flip_flops: HashMap<String, FlipFlop>,
    conjunctions: HashMap<String, Conjunction>,
    rx_input_cycles: HashMap<String, Option<u64>>,
}

impl Server {
    fn from_str(input: &str) -> Result<Self> {
        let mut server = Server::default();
        for line in input.lines() {
            let (channel, rec_list) = line
                .split_once("->")
                .map(|(c, r)| {
                    let rec: Vec<String> =
                        r.trim().split(',').map(|r| r.trim().to_string()).collect();
                    (c.trim(), rec)
                })
                .unwrap();
            match &channel[0..1] {
                "b" => {
                    if channel == "broadcaster" {
                        if server
                            .recievers
                            .insert(channel.to_string(), rec_list)
                            .is_some()
                        {
                            return Err(anyhow!("duplicate channel name {} in input", channel));
                        }
                    } else {
                        return Err(anyhow!("bad broadcaster input"));
                    }
                }
                "%" => {
                    let label = &channel[1..];
                    if server
                        .recievers
                        .insert(label.to_string(), rec_list)
                        .is_some()
                    {
                        return Err(anyhow!("duplicate channel name {} in input", label));
                    }
                    server
                        .flip_flops
                        .insert(label.to_string(), FlipFlop::default());
                }
                "&" => {
                    let label = &channel[1..];
                    if server
                        .recievers
                        .insert(label.to_string(), rec_list)
                        .is_some()
                    {
                        return Err(anyhow!("duplicate channel name {} in input", label));
                    }
                    server
                        .conjunctions
                        .insert(label.to_string(), Conjunction::default());
                }
                _ => return Err(anyhow!("bad input")),
            }
        }
        // filter recievers for input of conjunctions
        for (label, conjunction) in server.conjunctions.iter_mut() {
            for (rec_label, rec_list) in server.recievers.iter() {
                if rec_list.contains(label) {
                    conjunction.memory.insert(rec_label.to_owned(), false);
                }
            }
        }
        // get recievers of rx seeder (with some plausibility checks)
        let rx = String::from("rx");
        if !server
            .recievers
            .values()
            .any(|rec_list| rec_list.contains(&rx))
        {
            // test data, which does not contain rx
            return Ok(server);
        }
        if server
            .recievers
            .values()
            .filter(|rec_list| rec_list.contains(&rx))
            .count()
            > 1
        {
            return Err(anyhow!("too many or no inputs for rx"));
        }
        let rx_feeder = server
            .recievers
            .iter()
            .find(|(_, rec_list)| rec_list.contains(&rx))
            .unwrap()
            .0;
        if !server.conjunctions.keys().any(|k| k == rx_feeder) {
            return Err(anyhow!("rx feed is not a conjunction"));
        }
        // get nodes, who feed rx_feeder and check, if they are all conjunctions
        for (forxf, rec_list) in server
            .recievers
            .iter()
            .filter(|(_, rec)| rec.contains(rx_feeder))
        {
            if rec_list.len() != 1 {
                return Err(anyhow!("too many outputs for rx feeder"));
            }
            if !server.conjunctions.keys().any(|k| k == forxf) {
                return Err(anyhow!(
                    "feed {} into rx feeder is not a conjunction",
                    forxf
                ));
            }
            server.rx_input_cycles.insert(forxf.to_owned(), None);
        }
        Ok(server)
    }
    fn send(&mut self, mes: Message) {
        if mes.pulse {
            self.high_pulse_count += 1;
        } else {
            self.low_pulse_count += 1;
        }
        self.queue.push_back(mes);
    }
    fn push_button(&mut self) {
        self.send(Message::new_button_push());
    }
    fn recieve_message(&mut self) -> Option<Message> {
        self.queue.pop_front()
    }
    fn pulse_value(&self) -> u64 {
        self.low_pulse_count * self.high_pulse_count
    }
    fn get_recievers(&self, sender: &String) -> Result<Vec<String>> {
        match self.recievers.get(sender) {
            Some(rec) => Ok(rec.clone()),
            None => Err(anyhow!("internal error unknown sender {}", sender)),
        }
    }
    fn run(&mut self) -> Result<(u64, u64)> {
        let max_button_pushes: u64 = 10_000;
        let button_pushes_task1: u64 = 1_000;
        let mut counter = 0;
        let mut pulse_count = 0;
        while counter < max_button_pushes {
            // cycle count
            counter += 1;
            // push button
            self.push_button();
            while let Some(mes) = self.recieve_message() {
                if self.rx_input_cycles.contains_key(&mes.sender)
                    && mes.pulse
                    && let Some(cycle) = self.rx_input_cycles.get_mut(&mes.sender)
                    && cycle.is_none()
                {
                    *cycle = Some(counter);
                }
                if mes.reciever == "broadcaster" {
                    for rec_label in self.get_recievers(&mes.reciever)?.iter() {
                        self.send(Message::new_pulse(
                            mes.reciever.to_owned(),
                            rec_label.to_owned(),
                            false,
                        ));
                    }
                }
                if let Some(ff) = self.flip_flops.get_mut(&mes.reciever)
                    && let Some(pulse) = ff.recieve_pulse(mes.pulse)
                {
                    for rec_label in self.get_recievers(&mes.reciever)?.iter() {
                        self.send(Message::new_pulse(
                            mes.reciever.to_owned(),
                            rec_label.to_owned(),
                            pulse,
                        ));
                    }
                }
                if let Some(con) = self.conjunctions.get_mut(&mes.reciever) {
                    let pulse = con.recieve_pulse(mes.pulse, &mes.sender)?;
                    for rec_label in self.get_recievers(&mes.reciever)?.iter() {
                        self.send(Message::new_pulse(
                            mes.reciever.to_owned(),
                            rec_label.to_owned(),
                            pulse,
                        ));
                    }
                }
            }
            if counter == button_pushes_task1 {
                // count pulse after specified button pushes of task 1
                pulse_count = self.pulse_value();
            }
            if counter > button_pushes_task1 && self.rx_input_cycles.values().all(|c| c.is_some()) {
                break;
            }
        }

        let mut lcm_button_push = 1;
        for cycle in self.rx_input_cycles.values().map(|v| v.unwrap()) {
            lcm_button_push = lcm(lcm_button_push, cycle);
        }

        Ok((pulse_count, lcm_button_push))
    }
}

pub fn day_20() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_20.txt");
    let mut server = Server::from_str(input)?;
    let (result_part1, result_part2) = server.run()?;
    println!("result day 20 part 1: {}", result_part1);
    assert_eq!(result_part1, 825_167_435);
    println!("result day 20 part 2: {}", result_part2);
    assert_eq!(result_part2, 225_514_321_828_633);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example1_part1() -> Result<()> {
        let input = "broadcaster -> a, b, c\n\
                           %a -> b\n\
                           %b -> c\n\
                           %c -> inv\n\
                           &inv -> a";
        let mut server = Server::from_str(input)?;
        let (result_part1, _) = server.run()?;
        println!("result day 20 example 1 part 1: {}", result_part1);
        assert_eq!(result_part1, 32_000_000);
        Ok(())
    }

    #[test]
    fn test_example2_part1() -> Result<()> {
        let input = "broadcaster -> a\n\
                           %a -> inv, con\n\
                           &inv -> b\n\
                           %b -> con\n\
                           &con -> output";
        let mut server = Server::from_str(input)?;
        let (result_part1, _) = server.run()?;
        println!("result day 20 example 1 part 1: {}", result_part1);
        assert_eq!(result_part1, 11_687_500);
        Ok(())
    }
}
