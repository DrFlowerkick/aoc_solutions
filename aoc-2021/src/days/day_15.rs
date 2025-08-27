//!day_15.rs

use anyhow::Result;
use my_lib::{my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::{HashMap, VecDeque, hash_map::Entry};

#[cfg(any(feature = "long-run-time", test))]
type BigCave = MapPoint<5, 5>;

#[derive(Clone, Copy, Default)]
struct RiskLevel(u32);

impl From<char> for RiskLevel {
    fn from(value: char) -> Self {
        RiskLevel(value.to_digit(10).unwrap())
    }
}

impl RiskLevel {
    #[cfg(any(feature = "long-run-time", test))]
    fn get_level(&self, big_cave: BigCave) -> u32 {
        let distance = big_cave.distance((0, 0).into()) as u32;
        let mut level = self.0 + distance;
        if level > 9 {
            level %= 9;
        }
        level
    }
}

struct ChallengeInput<const X: usize, const Y: usize> {
    chitons: MyMap2D<RiskLevel, X, Y>,
}

impl<const X: usize, const Y: usize> From<&str> for ChallengeInput<X, Y> {
    fn from(value: &str) -> Self {
        ChallengeInput {
            chitons: MyMap2D::from(value),
        }
    }
}

impl<const X: usize, const Y: usize> ChallengeInput<X, Y> {
    fn solution_part_1(&self) -> u32 {
        let mut seen: HashMap<MapPoint<X, Y>, u32> = HashMap::new();
        let mut visit: VecDeque<(usize, MapPoint<X, Y>, u32)> = VecDeque::new();
        let mut lowest_risk: Option<(usize, u32)> = None;
        visit.push_back((0, (0, 0).into(), 0));
        while let Some((step, point, risk)) = visit.pop_front() {
            if point == MapPoint::SE {
                match lowest_risk {
                    Some((_, total_risk)) => {
                        if risk < total_risk {
                            lowest_risk = Some((step, risk));
                        }
                    }
                    None => {
                        lowest_risk = Some((step, risk));
                    }
                }
                continue;
            }
            // check if already visited with lower or equal risk
            match seen.entry(point) {
                Entry::Occupied(mut entry) => {
                    if risk < *entry.get() {
                        *entry.get_mut() = risk;
                    } else {
                        // same point was visited with same number of steps but equal or higher risk
                        continue;
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(risk);
                }
            }
            // check if end was reached with same number of steps and lower risk
            if let Some((steps_taken, total_risk)) = lowest_risk
                && step >= steps_taken
                && risk >= total_risk
            {
                continue;
            }
            // visit neighbors
            for (neighbor, _, neighbor_risk) in self.chitons.iter_neighbors(point) {
                visit.push_back((step + 1, neighbor, risk + neighbor_risk.0));
            }
        }
        lowest_risk.unwrap().1
    }
    #[cfg(any(feature = "long-run-time", test))]
    fn solution_part_2(&self) -> u32 {
        let mut seen: HashMap<(MapPoint<X, Y>, BigCave), u32> = HashMap::new();
        let mut visit: VecDeque<(usize, MapPoint<X, Y>, BigCave, u32)> = VecDeque::new();
        let mut lowest_risk: Option<(usize, u32)> = None;
        visit.push_back((0, (0, 0).into(), (0, 0).into(), 0));
        while let Some((step, point, big_cave, risk)) = visit.pop_front() {
            if point == MapPoint::SE && big_cave == BigCave::SE {
                match lowest_risk {
                    Some((_, total_risk)) => {
                        if risk < total_risk {
                            lowest_risk = Some((step, risk));
                        }
                    }
                    None => {
                        lowest_risk = Some((step, risk));
                    }
                }
                continue;
            }
            // check if already visited with lower or equal risk
            match seen.entry((point, big_cave)) {
                Entry::Occupied(mut entry) => {
                    if risk < *entry.get() {
                        *entry.get_mut() = risk;
                    } else {
                        // same point was visited with same number of steps but equal or higher risk
                        continue;
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(risk);
                }
            }
            // check if end was reached with same number of steps and lower risk
            if let Some((steps_taken, total_risk)) = lowest_risk
                && step >= steps_taken
                && risk >= total_risk
            {
                continue;
            }
            // visit neighbors
            for (neighbor, _, neighbor_risk) in self.chitons.iter_neighbors(point) {
                visit.push_back((
                    step + 1,
                    neighbor,
                    big_cave,
                    risk + neighbor_risk.get_level(big_cave),
                ));
            }
            match (point.x(), big_cave.x()) {
                (0, bc) if bc > 0 => {
                    // point is on west border, west is next big cave
                    let neighbor = point.invert_x();
                    let neighbor_risk = self.chitons.get(neighbor);
                    let new_big_cave = big_cave.backward_x().unwrap();
                    visit.push_back((
                        step + 1,
                        neighbor,
                        new_big_cave,
                        risk + neighbor_risk.get_level(new_big_cave),
                    ));
                }
                (p, bc) if p == X - 1 && bc < 4 => {
                    // point is on east border, east is next big cave
                    let neighbor = point.invert_x();
                    let neighbor_risk = self.chitons.get(neighbor);
                    let new_big_cave = big_cave.forward_x().unwrap();
                    visit.push_back((
                        step + 1,
                        neighbor,
                        new_big_cave,
                        risk + neighbor_risk.get_level(new_big_cave),
                    ));
                }
                _ => (),
            }
            match (point.y(), big_cave.y()) {
                (0, bc) if bc > 0 => {
                    // point is on north border, north is next big cave
                    let neighbor = point.invert_y();
                    let neighbor_risk = self.chitons.get(neighbor);
                    let new_big_cave = big_cave.backward_y().unwrap();
                    visit.push_back((
                        step + 1,
                        neighbor,
                        new_big_cave,
                        risk + neighbor_risk.get_level(new_big_cave),
                    ));
                }
                (p, bc) if p == Y - 1 && bc < 4 => {
                    // point is on south border, south is next big cave
                    let neighbor = point.invert_y();
                    let neighbor_risk = self.chitons.get(neighbor);
                    let new_big_cave = big_cave.forward_y().unwrap();
                    visit.push_back((
                        step + 1,
                        neighbor,
                        new_big_cave,
                        risk + neighbor_risk.get_level(new_big_cave),
                    ));
                }
                _ => (),
            }
        }
        lowest_risk.unwrap().1
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_15.txt");
    let challenge = ChallengeInput::<100, 100>::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_15 part 1: {result_part1}");
    assert_eq!(result_part1, 447);

    #[cfg(any(feature = "long-run-time", test))]
    {
        let result_part2 = challenge.solution_part_2();
        println!("result day_15 part 2: {result_part2}");
        assert_eq!(result_part2, 2_825);
    }
    #[cfg(not(feature = "long-run-time"))]
    {
        println!("day 15 part 2 skipped because of long run time")
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_15_example.txt");
        let example = ChallengeInput::<10, 10>::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_15 part 1: {result_part1}");
        assert_eq!(result_part1, 40);

        let result_part2 = example.solution_part_2();
        println!("result day_15 part 2: {result_part2}");
        assert_eq!(result_part2, 315);

        Ok(())
    }
}
