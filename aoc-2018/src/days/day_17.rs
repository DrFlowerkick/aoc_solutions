//!day_17.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WaterState {
    Flowing,
    FlowOrSettle,
    OutFlowing,
    Settled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Water {
    state: WaterState,
    distance: u64,
    pos: Point,
}

impl PartialOrd for Water {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Water {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.state.cmp(&other.state) {
            Ordering::Equal => {
                let cmp = match self.pos.y.cmp(&other.pos.y) {
                    Ordering::Equal => match self.distance.cmp(&other.distance) {
                        Ordering::Equal => self.pos.x.cmp(&other.pos.x),
                        cmp => cmp,
                    },
                    cmp => cmp,
                };
                if self.state == WaterState::Flowing {
                    cmp.reverse()
                } else {
                    cmp
                }
            }
            cmp => cmp,
        }
    }
}

impl Water {
    const SPRING: Point = Point { x: 500, y: 0 };
    fn new(pos: Point, distance: u64) -> Water {
        Water {
            state: WaterState::Flowing,
            distance,
            pos,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Ground {
    Sand,
    Water(WaterState),
    Clay,
    Oob,
}

struct ChallengeInput {
    clay: HashSet<Point>,
    max_y: i64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut clay = HashSet::new();
        let mut max_y = i64::MIN;
        for line in value.lines() {
            let (left, right) = line.split_once(", ").unwrap();
            let (coordinate, range) = if left.contains("..") {
                (right, left)
            } else {
                (left, right)
            };
            if coordinate.starts_with("x=") {
                let x: i64 = coordinate.strip_prefix("x=").unwrap().parse().unwrap();
                let (y_start, y_end) = range.strip_prefix("y=").unwrap().split_once("..").unwrap();
                let y_start: i64 = y_start.parse().unwrap();
                let y_end: i64 = y_end.parse().unwrap();
                for y in y_start..=y_end {
                    max_y = max_y.max(y);
                    clay.insert(Point::new(x, y));
                }
            } else {
                let y: i64 = coordinate.strip_prefix("y=").unwrap().parse().unwrap();
                max_y = max_y.max(y);
                let (x_start, x_end) = range.strip_prefix("x=").unwrap().split_once("..").unwrap();
                let x_start: i64 = x_start.parse().unwrap();
                let x_end: i64 = x_end.parse().unwrap();
                for x in x_start..=x_end {
                    clay.insert(Point::new(x, y));
                }
            }
        }
        ChallengeInput { clay, max_y }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let below_spring = Water::SPRING.add(Compass::S);
        let mut water_flow: BTreeSet<Water> = BTreeSet::new();
        water_flow.insert(Water::new(below_spring, 0));
        while let Some(water) = water_flow.first().copied()
            && water.state != WaterState::OutFlowing
        {
            match water.state {
                WaterState::Flowing => {
                    let south = water.pos.add(Compass::S);
                    match self.get_ground(south, &water_flow) {
                        Ground::Sand => {
                            water_flow.insert(Water::new(south, water.distance + 1));
                        }
                        Ground::Oob | Ground::Water(WaterState::OutFlowing) => {
                            let mut water = water_flow.pop_first().unwrap();
                            water.state = WaterState::OutFlowing;
                            water_flow.insert(water);
                        }
                        _ => {
                            let mut sand = false;
                            let mut out = false;
                            let west = water.pos.add(Compass::W);
                            match self.get_ground(west, &water_flow) {
                                Ground::Sand => {
                                    sand = true;
                                    water_flow.insert(Water::new(west, water.distance + 1));
                                }
                                Ground::Water(WaterState::OutFlowing) => {
                                    out = true;
                                }
                                _ => (),
                            }
                            let east = water.pos.add(Compass::E);
                            match self.get_ground(east, &water_flow) {
                                Ground::Sand => {
                                    sand = true;
                                    water_flow.insert(Water::new(east, water.distance + 1));
                                }
                                Ground::Water(WaterState::OutFlowing) => {
                                    out = true;
                                }
                                _ => (),
                            }
                            if sand {
                                continue;
                            }
                            // change state of water
                            let mut water = water_flow.pop_first().unwrap();
                            if out {
                                water.state = WaterState::OutFlowing;
                            } else {
                                water.state = WaterState::FlowOrSettle;
                            }
                            water_flow.insert(water);
                        }
                    }
                }
                WaterState::FlowOrSettle => {
                    let mut water = water_flow.pop_first().unwrap();
                    let mut out = false;
                    let west = water.pos.add(Compass::W);
                    match self.get_ground(west, &water_flow) {
                        Ground::Water(WaterState::OutFlowing) => {
                            out = true;
                        }
                        _ => (),
                    }
                    let east = water.pos.add(Compass::E);
                    match self.get_ground(east, &water_flow) {
                        Ground::Water(WaterState::OutFlowing) => {
                            out = true;
                        }
                        _ => (),
                    }
                    // change state of water
                    if out {
                        water.state = WaterState::OutFlowing;
                    } else {
                        water.state = WaterState::Settled;
                    }
                    water_flow.insert(water);
                }
                _ => {}
            }
        }
        water_flow.len()
    }
    fn solution_part_2(&self) -> u64 {
        0
    }
    fn get_ground(&self, pos: Point, water_flow: &BTreeSet<Water>) -> Ground {
        if pos.y > self.max_y {
            return Ground::Oob;
        }
        if self.clay.contains(&pos) {
            return Ground::Clay;
        }
        if let Some(water) = water_flow.iter().find(|w| w.pos == pos) {
            return Ground::Water(water.state);
        }
        Ground::Sand
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_17.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_17 part 1: {result_part1}");
    //assert_eq!(result_part1, XXX);

    let result_part2 = challenge.solution_part_2();
    println!("result day_17 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_17() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_17_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_17 part 1: {result_part1}");
        assert_eq!(result_part1, 57);

        let result_part2 = example.solution_part_2();
        println!("result day_17 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }
}
