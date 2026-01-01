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
    min_y: i64,
    max_y: i64,
    clay: HashSet<Point>,
    buckets: Vec<(Point, Point)>,
    flowing_water: HashSet<Point>,
    settled_water: HashSet<Point>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut clay = HashSet::new();
        let mut min_y = i64::MAX;
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
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                    clay.insert(Point::new(x, y));
                }
            } else {
                let y: i64 = coordinate.strip_prefix("y=").unwrap().parse().unwrap();
                min_y = min_y.min(y);
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
            min_y,
            max_y,
            clay,
            buckets: Vec::new(),
            flowing_water: HashSet::new(),
            settled_water: HashSet::new(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&mut self) -> (usize, usize) {
        self.identify_buckets();
        let mut seen: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Point> = VecDeque::new();
        // add point below spring to queue
        queue.push_back(Point::new(500, self.min_y));
        while let Some(water) = queue.pop_front() {
            if seen.insert(water) {
                if self.is_in_bucket(&water) {
                    self.settled_water.insert(water);
                    [Compass::S, Compass::W, Compass::E]
                        .into_iter()
                        .map(|c| water.add(c))
                        .filter(|p| self.is_sand(p) && !seen.contains(p))
                        .for_each(|p| queue.push_back(p));
                } else {
                    self.flowing_water.insert(water);
                    // check if water flows out
                    let south = water.add(Compass::S);
                    if self.is_flowing_out(&south) {
                        continue;
                    }
                    let check_sides = if self.is_sand(&south) {
                        if !seen.contains(&south) {
                            queue.push_back(south);
                        }
                        self.is_in_bucket(&south)
                    } else {
                        true
                    };

                    if check_sides {
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
            }
        }
        (seen.len(), self.settled_water.len())
    }
    fn identify_buckets(&mut self) {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut buckets: Vec<(Point, Point, Point)> = Vec::new();
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

                // get bucket end points
                let min_x = bucket.iter().map(|p| p.x).min().unwrap();
                let start_bucket = *bucket.iter().filter(|p| p.x == min_x).min().unwrap();
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
                let bottom_left = Point::new(end_bucket.x, max_y);
                buckets.push((start_bucket, end_bucket, bottom_left));
            }
        }
        // check for overlapping
        for (index, bucket_a) in buckets.iter().enumerate() {
            for bucket_b in buckets.iter().skip(index + 1) {
                let (big, small) = if bucket_a.0.x < bucket_b.0.x && bucket_b.1.x < bucket_a.1.x {
                    // b maybe in a
                    (bucket_a, bucket_b)
                } else if bucket_b.0.x < bucket_a.0.x && bucket_a.1.x < bucket_b.1.x {
                    // a maybe in b
                    (bucket_b, bucket_a)
                } else {
                    // no overlap
                    continue;
                };
                // if small bottom does not stick into big bucket, there is no overlap
                let big_min_y = big.0.y.min(big.1.y);
                let small_min_y = small.0.y.min(small.1.y);
                if small.2.y < big_min_y || small_min_y > big.2.y {
                    // no overlap
                    continue;
                }
                // if small is completely inside big, ignore it
                if big_min_y < small_min_y {
                    // small inside big
                    continue;
                }
                // small and big overlap -> cerate to virtuell buckets
                let left_bucket_bottom_right = Point::new(small.0.x, big.2.y);
                self.buckets.push((big.0, left_bucket_bottom_right));
                let right_bucket_top_left = Point::new(small.1.x, big.1.y);
                self.buckets.push((right_bucket_top_left, big.2));
            }
            // add bucket_a to bucket list
            let top_left = Point::new(bucket_a.0.x, bucket_a.0.y.max(bucket_a.1.y));
            self.buckets.push((top_left, bucket_a.2));
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
        self.buckets
            .iter()
            .any(|(tl, rb)| tl.x <= pos.x && pos.x <= rb.x && tl.y <= pos.y && pos.y <= rb.y)
    }
    fn _debug_print(&self) {
        let min_x = self
            .clay
            .iter()
            .chain(self.flowing_water.iter())
            .chain(self.settled_water.iter())
            .map(|p| p.x)
            .min()
            .unwrap();
        let max_x = self
            .clay
            .iter()
            .chain(self.flowing_water.iter())
            .chain(self.settled_water.iter())
            .map(|p| p.x)
            .max()
            .unwrap();
        println!();

        for y in 0..=self.max_y {
            //for y in (self.max_y/2)-5..=self.max_y {
            //for y in 0..=self.max_y / 2 {
            for x in min_x..=max_x {
                let pos = Point::new(x, y);
                if pos == Point::new(500, 0) {
                    print!("+")
                } else if self.buckets.iter().any(|(tl, br)| *tl == pos || *br == pos) {
                    print!("B")
                } else if self.clay.contains(&pos) {
                    print!("#");
                } else if self.flowing_water.contains(&pos) {
                    print!("|");
                } else if self.settled_water.contains(&pos) {
                    print!("~");
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

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_17 part 1: {result_part1}");
    assert_eq!(result_part1, 27_206);

    println!("result day_17 part 2: {result_part2}");
    assert_eq!(result_part2, 21_787);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_buckets() {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_17_example.txt");
        let mut example = ChallengeInput::from(input);

        example.identify_buckets();
        dbg!(&example.buckets);
        example._debug_print();
        let pos = Point::new(500, 2);
        assert!(!example.is_in_bucket(&pos));

        let pos = Point::new(500, 3);
        assert!(example.is_in_bucket(&pos));
    }

    #[test]
    fn test_example_day_17() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_17_example.txt");
        let mut example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_17 part 1: {result_part1}");
        assert_eq!(result_part1, 57);

        println!("result day_17 part 2: {result_part2}");
        assert_eq!(result_part2, 29);
        example._debug_print();

        Ok(())
    }
}
