//!day_15.rs

use super::day_05::IntCodeComputer;
use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
struct Cell {
    status: i64,
    distance: i64,
}

impl Cell {
    fn new(distance: i64) -> Self {
        Self {
            status: 3,
            distance,
        }
    }
}

struct ChallengeInput {
    code: IntCodeComputer,
    map: HashMap<Point, Cell>,
    bot: Point,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let bot = Point::new(0, 0);
        let mut map: HashMap<Point, Cell> = HashMap::new();
        map.insert(
            bot,
            Cell {
                status: 1,
                distance: 0,
            },
        );
        ChallengeInput {
            code: IntCodeComputer::from(value),
            map,
            bot,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> i64 {
        self.explore();
        self.map.values().find(|c| c.status == 2).unwrap().distance
    }
    fn solution_part_2(&self) -> i64 {
        let start = *self.map.iter().find(|(_, c)| c.status == 2).unwrap().0;
        let (seen, unexplored) = self.generate_distance_map(start);
        assert!(unexplored.is_none());
        *seen.values().max().unwrap()
    }
    fn explore(&mut self) {
        if let Some(neighbor) = self.add_neighbors() {
            // if neighbor is wall, bot remains on current position
            self.move_one_step(neighbor);
            self.explore();
        } else {
            if !self.map.iter().any(|(_, c)| c.status == 3) {
                // no more unexplored cells left
                return;
            }
            let path_to_next_unexplored = self.find_shortest_path_to_nearest_unexplored(self.bot);
            for step in path_to_next_unexplored.into_iter() {
                self.move_one_step(step);
            }
            self.explore();
        }
    }
    fn add_neighbors(&mut self) -> Option<Point> {
        // add new unexplored neighbors (if any) and returns first unexplored neighbor with
        // 1.) shortest distance or
        // 2.) min axis value (if shortest distance is equal)
        // returns None, if no unexplored neighbor exists
        let distance = self.map.get(&self.bot).unwrap().distance;
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|d| self.bot.add(d))
            .map(|neighbor| {
                (
                    neighbor,
                    // insert new neighbor, if key does not exist
                    *self.map.entry(neighbor).or_insert(Cell::new(distance + 1)),
                )
            })
            .filter(|(_, c)| c.status == 3)
            .min_by(
                |(n_1, c_1), (n_2, c_2)| match c_1.distance.cmp(&c_2.distance) {
                    Ordering::Equal => {
                        let min_axis_1 = n_1.x.abs().min(n_1.y.abs());
                        let min_axis_2 = n_2.x.abs().min(n_2.y.abs());
                        min_axis_1.cmp(&min_axis_2)
                    }
                    ord => ord,
                },
            )
            .map(|(p, _)| p)
    }
    fn move_one_step(&mut self, neighbor: Point) {
        let dir = match self.bot.subtract(neighbor) {
            Point { x: 0, y: 1 } => 1,
            Point { x: 0, y: -1 } => 2,
            Point { x: -1, y: 0 } => 3,
            Point { x: 1, y: 0 } => 4,
            _ => unreachable!(),
        };
        let status = match self.code.run_int_code(&[dir]) {
            Ok(out) => out.expect("unexpected halt"),
            Err(err) => panic!("{err}"),
        };
        self.map.get_mut(&neighbor).unwrap().status = status;
        match status {
            0 => {
                // wall -> do not move
            }
            1 | 2 => self.bot = neighbor,
            _ => panic!("unexpected status of int code"),
        }
    }
    fn find_shortest_path_to_nearest_unexplored(&mut self, start: Point) -> VecDeque<Point> {
        let (seen, end) = self.generate_distance_map(start);
        let end = end.expect("there should always be one unexplored end.");
        let mut distance = *seen.get(&end).unwrap();
        let mut current = end;
        let mut path: VecDeque<Point> = VecDeque::new();
        while distance > 0 {
            path.push_front(current);
            current = [(0, 1), (1, 0), (0, -1), (-1, 0)]
                .into_iter()
                .map(|dir| current.add(dir))
                .filter(|n| seen.contains_key(n))
                .find(|n| *seen.get(n).unwrap() == distance - 1)
                .unwrap();
            distance -= 1;
        }
        path
    }
    fn generate_distance_map(&self, start: Point) -> (HashMap<Point, i64>, Option<Point>) {
        // stops at first encountered unexplored cell
        let mut seen: HashMap<Point, i64> = HashMap::new();
        let mut visit: VecDeque<(Point, i64)> = VecDeque::new();
        visit.push_back((start, 0));
        while let Some((current, distance)) = visit.pop_front() {
            seen.insert(current, distance);
            if self.map.get(&current).unwrap().status == 3 {
                return (seen, Some(current));
            }
            for neighbor in [(0, 1), (1, 0), (0, -1), (-1, 0)]
                .into_iter()
                .map(|dir| current.add(dir))
                .filter(|p| self.map.get(p).unwrap().status != 0)
                .filter(|p| !seen.contains_key(p))
            {
                visit.push_back((neighbor, distance + 1));
            }
        }
        // no more unexplored cells left
        (seen, None)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_15.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_15 part 1: {result_part1}");
    assert_eq!(result_part1, 282);

    let result_part2 = challenge.solution_part_2();
    println!("result day_15 part 2: {result_part2}");
    assert_eq!(result_part2, 286);

    Ok(())
}

#[cfg(test)]
mod tests {
    // no challenge example
    /*use super::*;

    #[test]
    fn test_example_day_15() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_15_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_15 part 1: {result_part1}");
        //assert_eq!(result_part1, XXX);

        let result_part2 = example.solution_part_2();
        println!("result day_15 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }*/
}
