//!day_17.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Room {
    pos: Point,
    path: String,
}

impl PartialOrd for Room {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Room {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.path.len().cmp(&other.path.len()) {
            Ordering::Equal => match self.pos.delta((3, 3)).cmp(&other.pos.delta((3, 3))) {
                Ordering::Equal => self.path.cmp(&other.path),
                cmp => cmp,
            },
            cmp => cmp,
        }
    }
}

impl Room {
    fn next_possible_rooms(&self) -> impl Iterator<Item = Self> {
        let data = md5::compute(&self.path);
        Compass::cardinals().into_iter().filter_map(move |c| {
            let next_pos = self.pos.add(c);
            let open = match c {
                Compass::N => data[0] & 0xf0 > 0xa0,
                Compass::S => data[0] & 0x0f > 0x0a,
                Compass::W => data[1] & 0xf0 > 0xa0,
                Compass::E => data[1] & 0x0f > 0x0a,
                _ => unreachable!(),
            };
            if open && next_pos.x >= 0 && next_pos.x < 4 && next_pos.y >= 0 && next_pos.y < 4 {
                let mut next_room = Room {
                    pos: next_pos,
                    path: self.path.clone(),
                };
                let dir = match c {
                    Compass::N => 'U',
                    Compass::E => 'R',
                    Compass::S => 'D',
                    Compass::W => 'L',
                    _ => unreachable!(),
                };
                next_room.path.push(dir);
                Some(next_room)
            } else {
                None
            }
        })
    }
}

struct ChallengeInput<'a> {
    seed: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        ChallengeInput { seed: value }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> String {
        let initial_room = Room {
            path: self.seed.to_string(),
            pos: (0, 0).into(),
        };
        let mut sort_queue: BTreeSet<Room> = BTreeSet::new();
        sort_queue.insert(initial_room);
        while let Some(current) = sort_queue.pop_first() {
            if current.pos.x == 3 && current.pos.y == 3 {
                let path = current.path.strip_prefix(self.seed).unwrap();
                return path.to_string();
            }
            for next in current.next_possible_rooms() {
                sort_queue.insert(next);
            }
        }
        "".into()
    }
    fn solution_part_2(&self) -> usize {
        let mut max_steps = 0;
        let initial_room = Room {
            path: self.seed.to_string(),
            pos: (0, 0).into(),
        };
        let mut queue: Vec<Room> = Vec::new();
        queue.push(initial_room);
        while let Some(current) = queue.pop() {
            if current.pos.x == 3 && current.pos.y == 3 {
                max_steps = max_steps.max(current.path.len());
                continue;
            }
            for next in current.next_possible_rooms() {
                queue.push(next);
            }
        }
        max_steps - self.seed.len()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_17.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_17 part 1: {result_part1}");
    assert_eq!(result_part1, "RLDRUDRDDR");

    let result_part2 = challenge.solution_part_2();
    println!("result day_17 part 2: {result_part2}");
    assert_eq!(result_part2, 498);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_no_solution() {
        let no_solution = ChallengeInput { seed: "hijkl" };
        assert_eq!(no_solution.solution_part_1(), "");
    }

    #[test]
    fn test_example_day_17() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_17_example.txt");

        let solutions = [
            ("DDRRRD", 370_usize),
            ("DDUDRLRRUDRD", 492),
            ("DRURDRUDDLLDLUURRDULRLDUUDDDRR", 830),
        ];

        for (line, (solution_part_1, solution_part_2)) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let result_part1 = example.solution_part_1();
            println!("result day_17 part 1: {result_part1}");
            assert_eq!(result_part1, solution_part_1);

            let result_part2 = example.solution_part_2();
            println!("result day_17 part 2: {result_part2}");
            assert_eq!(result_part2, solution_part_2);
        }

        Ok(())
    }
}
