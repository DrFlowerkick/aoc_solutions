//!day_23.rs

use super::day_05::{IntCodeComputer, IntOut};
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender, error::TryRecvError};

struct Nic {}

impl Nic {
    fn boot(
        mut code: IntCodeComputer,
        address: i64,
        out_sender: UnboundedSender<Result<(i64, IntOut), String>>,
        sleep_if_empty: Duration,
    ) -> UnboundedSender<i64> {
        let (tx, receiver) = mpsc::unbounded_channel();

        thread::spawn(move || {
            code.set_id(address);
            code.run_int_code_with_mpsc(receiver, out_sender, Some(-1), sleep_if_empty)
                .unwrap();
        });
        // send address to initialize Nic code
        tx.send(address).unwrap();
        tx
    }
}

struct ChallengeInput {
    code: IntCodeComputer,
    rx: UnboundedReceiver<Result<(i64, IntOut), String>>,
    tx: UnboundedSender<Result<(i64, IntOut), String>>,
    sleep_if_empty: Duration,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        ChallengeInput {
            code: IntCodeComputer::from(value),
            rx,
            tx,
            sleep_if_empty: Duration::from_millis(3),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> Result<i64> {
        let nic_input_senders: HashMap<i64, UnboundedSender<i64>> = (0..50)
            .map(|addr| {
                (
                    addr,
                    Nic::boot(
                        self.code.clone(),
                        addr,
                        self.tx.clone(),
                        self.sleep_if_empty,
                    ),
                )
            })
            .collect();
        let mut nic_queues: HashMap<i64, VecDeque<i64>> = HashMap::new();
        for key in nic_input_senders.keys() {
            nic_queues.insert(*key, VecDeque::new());
        }
        loop {
            while let Ok(message) = self.rx.try_recv() {
                match message {
                    Ok((addr, IntOut::Out(out))) => {
                        if let Some(nic_queue) = nic_queues.get_mut(&addr) {
                            nic_queue.push_back(out);
                            if nic_queue.len() == 3 {
                                let target_addr = nic_queue.pop_front().unwrap();
                                if let Some(target_sender) = nic_input_senders.get(&target_addr) {
                                    while let Some(input) = nic_queue.pop_front() {
                                        target_sender.send(input).unwrap();
                                    }
                                } else if target_addr == 255 {
                                    nic_queue.pop_front();
                                    return Ok(nic_queue.pop_front().unwrap());
                                }
                            }
                        }
                    }
                    Ok((_, IntOut::Halt)) => panic!("unexpected halt of Int Computer."),
                    Ok((_, IntOut::None)) => unreachable!(),
                    Err(err) => panic!("Int Computer returned error:\n{err}"),
                }
            }
        }
    }
    fn solution_part_2(&mut self) -> Result<i64> {
        let nic_input_senders: HashMap<i64, UnboundedSender<i64>> = (0..50)
            .map(|addr| {
                (
                    addr,
                    Nic::boot(
                        self.code.clone(),
                        addr,
                        self.tx.clone(),
                        self.sleep_if_empty,
                    ),
                )
            })
            .collect();
        let mut nic_queues: HashMap<i64, VecDeque<i64>> = HashMap::new();
        for key in nic_input_senders.keys() {
            nic_queues.insert(*key, VecDeque::new());
        }
        let mut current_nat = (0, 0);
        let mut seen_nat_send_to_zero: HashSet<(i64, i64)> = HashSet::new();
        let mut check_queues = -1;
        loop {
            match self.rx.try_recv() {
                Ok(Ok((addr, IntOut::Out(out)))) => {
                    if let Some(nic_queue) = nic_queues.get_mut(&addr) {
                        nic_queue.push_back(out);
                        if nic_queue.len() == 3 {
                            let target_addr = nic_queue.pop_front().unwrap();
                            if let Some(target_sender) = nic_input_senders.get(&target_addr) {
                                while let Some(input) = nic_queue.pop_front() {
                                    target_sender.send(input).unwrap();
                                }
                            } else if target_addr == 255 {
                                current_nat.0 = nic_queue.pop_front().unwrap();
                                current_nat.1 = nic_queue.pop_front().unwrap();

                                check_queues = 0;
                            }
                        }
                    }
                }
                Ok(Ok((_, IntOut::Halt))) => panic!("unexpected halt of Int Computer."),
                Ok(Ok((_, IntOut::None))) => unreachable!(),
                Ok(Err(err)) => panic!("Int Computer returned error:\n{err}"),
                Err(TryRecvError::Empty) => {
                    if check_queues == 10 {
                        check_queues = -1;
                        if nic_queues.values().all(|q| q.is_empty()) {
                            // all queues are empty and no one is sending -> idle state
                            if !seen_nat_send_to_zero.insert(current_nat) {
                                return Ok(current_nat.1);
                            }
                            if let Some(sender_zero) = nic_input_senders.get(&0) {
                                sender_zero.send(current_nat.0).unwrap();
                                sender_zero.send(current_nat.1).unwrap();
                            }
                        }
                    } else if check_queues >= 0 {
                        // wait a few cycles and see, if any new packets are send over the nic net
                        thread::sleep(self.sleep_if_empty);
                        check_queues += 1;
                    }
                }
                Err(TryRecvError::Disconnected) => return Ok(0),
            }
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_23.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1()?;
    println!("result day_23 part 1: {result_part1}");
    assert_eq!(result_part1, 26_744);

    let result_part2 = challenge.solution_part_2()?;
    println!("result day_23 part 2: {result_part2}");
    assert_eq!(result_part2, 19_498);

    Ok(())
}

#[cfg(test)]
mod tests {
    /* int code challenge does not provide example
    use super::*;

    #[test]
    fn test_example_day_23() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_23_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_23 part 1: {result_part1}");
        //assert_eq!(result_part1, XXX);

        let result_part2 = example.solution_part_2();
        println!("result day_23 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    } */
}
