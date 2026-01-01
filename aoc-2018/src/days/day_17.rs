//!day_17.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Ground {
    Sand,
    FlowingWater,
    SettledWater,
    Clay,
    Oob,
}

#[derive(Debug)]
struct ChallengeInput {
    max_y: i64,
    clay: HashSet<Point>,
    buckets: Vec<(Point, Point)>,
    flowing_water: HashSet<Point>,
    settled_water: HashSet<Point>,
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
        ChallengeInput {
            max_y,
            clay,
            buckets: Vec::new(),
            flowing_water: HashSet::new(),
            settled_water: HashSet::new(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> usize {
        self.identify_buckets();
        let mut seen: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Point> = VecDeque::new();
        // add point below spring to queue
        queue.push_back(Point::new(500, 1));
        let mut collected_water: Vec<Point> = Vec::new();
        let mut remaining_water: Vec<Point> = Vec::new();
        while !queue.is_empty() || !collected_water.is_empty() || !remaining_water.is_empty() {
            // collect water
            //dbg!("collect water");
            while let Some(water) = queue.pop_front() {
                if seen.contains(&water) {
                    continue;
                }
                seen.insert(water);
                collected_water.push(water);
                // check if water flows down
                let south = water.add(Compass::S);
                if self.is_sand(&south) && !seen.contains(&south) {
                    queue.push_back(south);
                } else if self.is_flowing_out(&south) {
                    continue;
                } else {
                    // check if water flows left or right
                    let west = water.add(Compass::W);
                    if self.is_sand(&west) && !seen.contains(&west) {
                        queue.push_back(west);
                    }
                    let east = water.add(Compass::E);
                    if self.is_sand(&east) && !seen.contains(&east) {
                        queue.push_back(east);
                    }
                }
            }
            //dbg!(&collected_water);
            // check if collected water is flowing out of bounds
            //dbg!("check if collected water is flowing out of bounds");
            while let Some(water) = collected_water.pop() {
                if [Compass::S, Compass::W, Compass::E]
                    .into_iter()
                    .map(|c| water.add(c))
                    .any(|pos| self.is_flowing_out(&pos))
                {
                    self.flowing_water.insert(water);
                } else {
                    // check if water flows left or right

                    if [Compass::W, Compass::E]
                        .into_iter()
                        .map(|c| water.add(c))
                        .any(|pos| self.is_sand(&pos) && !seen.contains(&pos))
                    {
                        queue.push_back(water);
                        // remove water from seen to recheck flowing of water
                        seen.remove(&water);
                        break;
                    }
                    remaining_water.push(water);
                }
            }
            //dbg!(&remaining_water);
            // check if remaining water is flowing out of bounds or settled
            //dbg!("check if remaining water is flowing out of bounds or settled");
            while let Some(water) = remaining_water.pop() {
                if [Compass::W, Compass::E]
                    .into_iter()
                    .map(|c| water.add(c))
                    .any(|pos| self.is_flowing_out(&pos))
                {
                    self.flowing_water.insert(water);
                } else {
                    self.settled_water.insert(water);
                }
            }
        }
        self.debug_print(&queue, &collected_water, &remaining_water);
        self.flowing_water.len() + self.settled_water.len()
    }
    fn solution_part_2(&self) -> u64 {
        0
    }
    fn identify_buckets(&mut self) {
        let mut seen: HashSet<Point> = HashSet::new();
        for clay in self.clay.iter() {
            if seen.insert(*clay) {
                let mut bucket: HashSet<Point> = HashSet::new();
                let mut queue: VecDeque<Point> = VecDeque::new();
                queue.push_back(*clay);
                while let Some(pos) = queue.pop_front() {
                    if bucket.insert(pos) {
                        seen.insert(pos);
                        Compass::cardinals()
                            .into_iter()
                            .map(|c| pos.add(c))
                            .filter(|p| self.clay.contains(p))
                            .for_each(|p| queue.push_back(p));
                    }
                }
                // get bucket size
                // min: top left with y pointing down
                let start_bucket = *bucket.iter().min().unwrap();
                let mut next = start_bucket;
                // down
                while bucket.contains(&next) {
                    next = next.add(Compass::S);
                }
                // right (correct one step too much down)
                next = next.add(Compass::N);
                let max_y = next.y;
                while bucket.contains(&next) {
                    next = next.add(Compass::E);
                }
                // up (correct one step to much right)
                next = next.add(Compass::W);
                while bucket.contains(&next) {
                    next = next.add(Compass::N);
                }
                // end of bucket: correct one step too much up
                let end_bucket = next.add(Compass::S);
                let min_y = start_bucket.y.min(end_bucket.y);
                let top_left = Point::new(start_bucket.x, min_y);
                let bottom_right = Point::new(end_bucket.x, max_y);
                self.buckets.push((top_left, bottom_right));
            }
        }
    }
    fn get_ground(&self, pos: &Point) -> Ground {
        if pos.y > self.max_y {
            Ground::Oob
        } else if self.flowing_water.contains(pos) {
            Ground::FlowingWater
        } else if self.settled_water.contains(pos) {
            Ground::SettledWater
        } else if self.clay.contains(pos) {
            Ground::Clay
        } else {
            Ground::Sand
        }
    }
    fn is_sand(&self, pos: &Point) -> bool {
        matches!(self.get_ground(pos), Ground::Sand)
    }
    fn is_flowing_out(&self, pos: &Point) -> bool {
        matches!(self.get_ground(pos), Ground::Oob | Ground::FlowingWater)
    }
    fn is_in_bucket(&self, pos: &Point) -> bool {
        self.buckets.iter().any(|(tl, rb)| tl <= pos && pos <= rb)
    }
    fn debug_print(
        &self,
        queue: &VecDeque<Point>,
        collected_water: &Vec<Point>,
        remaining_water: &Vec<Point>,
    ) {
        let min_x = self
            .clay
            .iter()
            .chain(self.flowing_water.iter())
            .chain(self.settled_water.iter())
            .chain(queue.iter())
            .chain(collected_water.iter())
            .chain(remaining_water.iter())
            .map(|p| p.x)
            .min()
            .unwrap();
        let max_x = self
            .clay
            .iter()
            .chain(self.flowing_water.iter())
            .chain(self.settled_water.iter())
            .chain(queue.iter())
            .chain(collected_water.iter())
            .chain(remaining_water.iter())
            .map(|p| p.x)
            .max()
            .unwrap();
        println!();
        for y in 0..=self.max_y {
            for x in min_x..=max_x {
                let pos = Point::new(x, y);
                if pos == Point::new(500, 0) {
                    print!("+")
                } else if self.clay.contains(&pos) {
                    print!("#");
                } else if self.flowing_water.contains(&pos) {
                    print!("|");
                } else if self.settled_water.contains(&pos) {
                    print!("~");
                } else if queue.contains(&pos) {
                    print!("q");
                } else if collected_water.contains(&pos) {
                    print!("c");
                } else if remaining_water.contains(&pos) {
                    print!("?");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_17.txt");
    let mut challenge = ChallengeInput::from(input);

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
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_17 part 1: {result_part1}");
        assert_eq!(result_part1, 57);

        let result_part2 = example.solution_part_2();
        println!("result day_17 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }
}
