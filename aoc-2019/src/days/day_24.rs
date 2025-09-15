//!day_24.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::{HashMap, HashSet, VecDeque};

struct ChallengeInput {
    bugs: MyMap2D<char, 5, 5>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            bugs: MyMap2D::from(value),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut seen: HashSet<MyMap2D<char, 5, 5>> = HashSet::new();
        let mut bugs = self.bugs;
        loop {
            if !seen.insert(bugs) {
                break;
            }
            let mut next = bugs;
            for (tile, value) in bugs.iter() {
                let adjacent_bugs = bugs
                    .iter_neighbors(tile)
                    .filter(|(_, _, v)| **v == '#')
                    .count();
                if *value == '#' {
                    if adjacent_bugs != 1 {
                        next.set(tile, '.');
                    }
                } else if adjacent_bugs == 1 || adjacent_bugs == 2 {
                    next.set(tile, '#');
                }
            }

            bugs = next;
        }
        bugs.iter()
            .enumerate()
            .filter(|(_, (_, v))| **v == '#')
            .map(|(i, _)| 2_u64.pow(i as u32))
            .sum()
    }
    fn solution_part_2(&self, mut minutes: usize) -> usize {
        // N, E, S, W around center
        let center_cardinals: [MapPoint<5, 5>; 4] =
            [(2, 1).into(), (3, 2).into(), (2, 3).into(), (1, 2).into()];
        let mut seen: HashMap<i64, MyMap2D<char, 5, 5>> = HashMap::new();
        seen.insert(0, self.bugs);
        while minutes > 0 {
            minutes -= 1;
            let mut next_seen: HashMap<i64, MyMap2D<char, 5, 5>> = HashMap::new();
            let mut queue: VecDeque<(MyMap2D<char, 5, 5>, i64)> =
                seen.iter().map(|(l, b)| (*b, *l)).collect();
            while let Some((bugs, level)) = queue.pop_front() {
                let mut next = bugs;
                for (tile, value) in bugs.iter().filter(|(p, _)| *p != (2, 2).into()) {
                    let mut adjacent_bugs = bugs
                        .iter_neighbors(tile)
                        .filter(|(_, _, v)| **v == '#')
                        .count();
                    // add bugs from inner and outer levels
                    adjacent_bugs += match tile.map_position() {
                        Compass::Center => {
                            // current position is inside of map
                            if let Some(center_pos_index) =
                                center_cardinals.iter().position(|p| *p == tile)
                            {
                                // get bugs of inner
                                let inner_level = level + 1;
                                if let Some(inner) = seen.get(&inner_level) {
                                    match center_pos_index {
                                        0 => inner.iter_row(0).filter(|(_, v)| **v == '#').count(),
                                        1 => {
                                            inner.iter_column(4).filter(|(_, v)| **v == '#').count()
                                        }
                                        2 => inner.iter_row(4).filter(|(_, v)| **v == '#').count(),
                                        3 => {
                                            inner.iter_column(0).filter(|(_, v)| **v == '#').count()
                                        }
                                        _ => unreachable!(),
                                    }
                                } else if *value == '#' {
                                    // inner does not exist yet: create it and push it to queue,
                                    // if current tile contains a bug
                                    let inner: MyMap2D<char, 5, 5> = MyMap2D::init('.');
                                    queue.push_back((inner, inner_level));
                                    0
                                } else {
                                    0
                                }
                            } else {
                                // only bugs on same level
                                0
                            }
                        }
                        _ => {
                            // current position is on outer edge of map -> get bugs of outer
                            Compass::cardinals()
                                .iter()
                                .filter(|c| tile.neighbor(**c).is_none())
                                .map(|outer_dir| {
                                    let outer_level = level - 1;
                                    if let Some(outer) = seen.get(&outer_level) {
                                        let outer_tile = match outer_dir {
                                            Compass::N => center_cardinals[0],
                                            Compass::E => center_cardinals[1],
                                            Compass::S => center_cardinals[2],
                                            Compass::W => center_cardinals[3],
                                            _ => unreachable!(),
                                        };
                                        (*outer.get(outer_tile) == '#') as usize
                                    } else if *value == '#' {
                                        // outer does not exist yet: create it and push it to queue,
                                        // if current tile contains a bug
                                        let outer: MyMap2D<char, 5, 5> = MyMap2D::init('.');
                                        queue.push_back((outer, outer_level));
                                        0
                                    } else {
                                        0
                                    }
                                })
                                .sum()
                        }
                    };
                    if *value == '#' {
                        if adjacent_bugs != 1 {
                            next.set(tile, '.');
                        }
                    } else if adjacent_bugs == 1 || adjacent_bugs == 2 {
                        next.set(tile, '#');
                    }
                }
                next_seen.insert(level, next);
            }
            seen = next_seen;
        }
        seen.values()
            .map(|b| b.iter().filter(|(_, v)| **v == '#').count())
            .sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_24.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_24 part 1: {result_part1}");
    assert_eq!(result_part1, 28_903_899);

    let result_part2 = challenge.solution_part_2(200);
    println!("result day_24 part 2: {result_part2}");
    assert_eq!(result_part2, 1_896);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_24() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_24_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_24 part 1: {result_part1}");
        assert_eq!(result_part1, 2_129_920);

        let result_part2 = example.solution_part_2(10);
        println!("result day_24 part 2: {result_part2}");
        assert_eq!(result_part2, 99);

        Ok(())
    }
}
