//!day_22.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::{BTreeSet, HashMap};

struct ChallengeInput {
    depth: i64,
    target: Point,
    cave: HashMap<Point, (i64, i64, i64)>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (depth, target) = value.split_once('\n').unwrap();
        let (x, y) = target
            .strip_prefix("target: ")
            .unwrap()
            .split_once(',')
            .unwrap();
        ChallengeInput {
            depth: depth.strip_prefix("depth: ").unwrap().parse().unwrap(),
            target: Point::new(x.parse().unwrap(), y.parse().unwrap()),
            cave: HashMap::new(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> i64 {
        self.extend_cave(1);
        self.cave.values().map(|(_, _, t)| *t).sum()
    }
    fn solution_part_2(&mut self) -> i64 {
        self.extend_cave(5);
        let mut seen: HashMap<(Point, i64), i64> = HashMap::new();
        seen.insert((Point::default(), 2), 0);
        let mut sorted_queue: BTreeSet<(i64, Point, i64)> = BTreeSet::new();
        sorted_queue.insert((0, Point::default(), 2));
        let mut min_time = i64::MAX;
        while let Some((time, pos, equipment)) = sorted_queue.pop_first() {
            if let Some(last_time) = seen.get(&(pos, equipment))
                && time > *last_time
            {
                continue;
            }
            if time > min_time {
                continue;
            }
            // cache pos
            seen.insert((pos, equipment), time);
            // check for target pos
            if pos == self.target {
                // check if torch
                if equipment == 2 {
                    // reached target with torch
                    min_time = min_time.min(time);
                } else {
                    sorted_queue.insert((time + 7, pos, 2));
                }
            } else {
                let current_allowed_equipment = self.allowed_equipment(pos);
                for next_pos in Compass::cardinals()
                    .into_iter()
                    .map(|c| pos.add(c))
                    .filter(|p| p.x >= 0 && p.y >= 0)
                {
                    if !self.cave.contains_key(&next_pos) {
                        continue;
                    }
                    for next_equipment in self
                        .allowed_equipment(next_pos)
                        .into_iter()
                        .filter(|e| current_allowed_equipment.contains(e))
                    {
                        let next_time = time + if equipment == next_equipment { 1 } else { 8 };
                        sorted_queue.insert((next_time, next_pos, next_equipment));
                    }
                }
            }
        }
        min_time
    }
    fn extend_cave(&mut self, factor: i64) {
        for y in 0..=self.target.y * factor {
            for x in 0..=self.target.x * factor {
                let tile = Point::new(x, y);
                if !self.cave.contains_key(&tile) {
                    self.cave.insert(tile, self.calc_cave_tile(tile));
                }
            }
        }
    }
    fn calc_cave_tile(&self, tile: Point) -> (i64, i64, i64) {
        let geologic_index = if tile == Point::default() || tile == self.target {
            0
        } else if tile.x == 0 {
            tile.y * 48_271
        } else if tile.y == 0 {
            tile.x * 16_807
        } else {
            let north = tile.add(Compass::N);
            let west = tile.add(Compass::W);
            self.cave.get(&north).unwrap().1 * self.cave.get(&west).unwrap().1
        };
        let erosion_level = (geologic_index + self.depth) % 20_183;
        let region_type = erosion_level % 3;
        assert!([0, 1, 2].contains(&region_type));
        (geologic_index, erosion_level, region_type)
    }
    fn allowed_equipment(&self, tile: Point) -> [i64; 2] {
        let region_type = self.cave.get(&tile).unwrap().2;
        match region_type {
            0 => [1, 2], // rocky: climbing gear (1) or torch (2)
            1 => [0, 1], // wet: neither (0) or climbing gear (1)
            2 => [0, 2], // narrow: neither (0) or torch (2)
            _ => unreachable!(),
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_22.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_22 part 1: {result_part1}");
    assert_eq!(result_part1, 6_208);

    let result_part2 = challenge.solution_part_2();
    println!("result day_22 part 2: {result_part2}");
    assert_eq!(result_part2, 1_039);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_22() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_22_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_22 part 1: {result_part1}");
        assert_eq!(result_part1, 114);

        let result_part2 = example.solution_part_2();
        println!("result day_22 part 2: {result_part2}");
        assert_eq!(result_part2, 45);

        Ok(())
    }
}
