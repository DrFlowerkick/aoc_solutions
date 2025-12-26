//!day_22.rs

use anyhow::Result;
use my_lib::{my_geometry::my_point::Point3D, my_pixels::my_box::Box3D};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    box3d: Box3D,
    status: bool,
    size: bool,
}

impl From<&str> for Cuboid {
    fn from(value: &str) -> Self {
        let (status, coordinates) = value.split_once(' ').unwrap();
        let status = status == "on";
        let coordinates: Vec<(i64, i64)> = coordinates
            .split(',')
            .filter_map(|c| c[2..].split_once(".."))
            .map(|(min, max)| (min.parse::<i64>().unwrap(), max.parse::<i64>().unwrap()))
            .collect();
        let left_front_bottom = Point3D {
            x: coordinates[0].0,
            y: coordinates[1].0,
            z: coordinates[2].0,
        };
        let right_back_top = Point3D {
            x: coordinates[0].1,
            y: coordinates[1].1,
            z: coordinates[2].1,
        };
        let box3d = Box3D::new(left_front_bottom, right_back_top);
        let part_1_cuboid = Box3D {
            left_front_bottom: Point3D {
                x: -50,
                y: -50,
                z: -50,
            },
            right_back_top: Point3D {
                x: 50,
                y: 50,
                z: 50,
            },
        };
        let size = box3d.intersect(part_1_cuboid).is_none();
        Cuboid {
            box3d,
            status,
            size,
        }
    }
}

impl Cuboid {
    fn get_new_booted(&self, ci: Cuboid) -> Option<Vec<Cuboid>> {
        // self: current booted cuboid
        // ci: new cuboid "on" instruction
        if !ci.status {
            return None;
        }
        self.box3d.split_intersecting(ci.box3d).map(
            |(_booted_intersection, _remaining_already_booted, new_ci_booted)| {
                new_ci_booted
                    .into_iter()
                    .map(|box3d| Cuboid {
                        box3d,
                        status: ci.status,
                        size: ci.size,
                    })
                    .collect()
            },
        )
    }
    fn get_remaining_booted(&self, ci: Cuboid) -> Option<Vec<Cuboid>> {
        // self: current booted cuboid
        // ci: new cuboid "off" instruction
        if ci.status {
            return None;
        }
        self.box3d.split_intersecting(ci.box3d).map(
            |(_booted_intersection, remaining_booted, _ci_remaining_off)| {
                remaining_booted
                    .into_iter()
                    .map(|box3d| Cuboid {
                        box3d,
                        status: ci.status,
                        size: ci.size,
                    })
                    .collect()
            },
        )
    }
}

struct ChallengeInput {
    cuboid_instructions: Vec<Cuboid>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            cuboid_instructions: value.lines().map(Cuboid::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.boot_cubes(false)
    }
    fn solution_part_2(&self) -> u64 {
        self.boot_cubes(true)
    }
    fn boot_cubes(&self, full: bool) -> u64 {
        let mut booted_cuboids: VecDeque<Cuboid> = VecDeque::new();
        for cuboid_instruction in self.cuboid_instructions.iter().filter(|c| !c.size || full) {
            // preparation
            let mut ci_queue: VecDeque<Cuboid> = vec![*cuboid_instruction].into();
            if cuboid_instruction.status {
                // add new booted blocks
                for bc in booted_cuboids.iter() {
                    let mut num_ci = ci_queue.len();
                    while num_ci > 0 {
                        num_ci -= 1;
                        let ci = ci_queue.pop_front().unwrap();
                        if let Some(remaining_ci) = bc.get_new_booted(ci) {
                            // push remaining on cuboids of ci to ci queue
                            remaining_ci
                                .into_iter()
                                .for_each(|rci| ci_queue.push_back(rci));
                        } else {
                            // no intersection, push ci back to queue for next booted cuboids
                            ci_queue.push_back(ci);
                        }
                    }
                }
                // push remaining ci on cuboids to booted_cuboids
                ci_queue
                    .into_iter()
                    .for_each(|ci| booted_cuboids.push_back(ci));
            } else {
                // remove booted cuboids with ci
                let mut num_bc = booted_cuboids.len();
                while num_bc > 0 {
                    num_bc -= 1;
                    let bc = booted_cuboids.pop_front().unwrap();
                    if let Some(remaining_booted) = bc.get_remaining_booted(*cuboid_instruction) {
                        // push remaining booted back to booted_cuboids
                        remaining_booted
                            .into_iter()
                            .for_each(|rb| booted_cuboids.push_back(rb));
                    } else {
                        // no intersection, keep booted cuboid
                        booted_cuboids.push_back(bc);
                    }
                }
            }
        }
        booted_cuboids.iter().filter_map(|c| c.box3d.size()).sum()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_22.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_22 part 1: {result_part1}");
    assert_eq!(result_part1, 553_201);

    let result_part2 = challenge.solution_part_2();
    println!("result day_22 part 2: {result_part2}");
    assert_eq!(result_part2, 1_263_946_820_845_866);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn verify_parsing() {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_22_example_2.txt");
        let example = ChallengeInput::from(input);

        let (num_changes_of_size, index_of_last_change, _) = example
            .cuboid_instructions
            .iter()
            .map(|c| c.size)
            .enumerate()
            .fold(
                (0, 0, false),
                |(mut count, mut c_index, last), (index, current)| {
                    if last ^ current {
                        count += 1;
                        c_index = index;
                    }
                    (count, c_index, current)
                },
            );
        assert_eq!(num_changes_of_size, 1);
        assert_eq!(index_of_last_change, 20);
        assert_eq!(example.cuboid_instructions.len(), 22);

        let input = include_str!("../../../../aoc_input/aoc-2021/day_22.txt");
        let challenge = ChallengeInput::from(input);

        let (num_changes_of_size, index_of_last_change, _) = challenge
            .cuboid_instructions
            .iter()
            .map(|c| c.size)
            .enumerate()
            .fold(
                (0, 0, false),
                |(mut count, mut c_index, last), (index, current)| {
                    if last ^ current {
                        count += 1;
                        c_index = index;
                    }
                    (count, c_index, current)
                },
            );
        assert_eq!(num_changes_of_size, 1);
        assert_eq!(index_of_last_change, 20);
        assert_eq!(challenge.cuboid_instructions.len(), 420);
    }

    #[test]
    fn test_example_1_day_22() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_22_example_1.txt");
        let example = ChallengeInput::from(input);

        // check first and second instruction
        let size_0 = example.cuboid_instructions[0].box3d.size().unwrap();
        let size_1 = example.cuboid_instructions[1].box3d.size().unwrap();
        assert_eq!(size_0, 27);
        assert_eq!(size_1, 27);

        let (intersection, remaining_0, remaining_1) = example.cuboid_instructions[0]
            .box3d
            .split_intersecting(example.cuboid_instructions[1].box3d)
            .unwrap();
        let size_intersection = intersection.size().unwrap();
        let size_remaining_0: u64 = remaining_0.iter().filter_map(|b| b.size()).sum();
        let size_remaining_1: u64 = remaining_1.iter().filter_map(|b| b.size()).sum();
        assert_eq!(size_intersection, 8);
        assert_eq!(size_remaining_0, 19);
        assert_eq!(size_remaining_1, 19);

        // check third instruction
        let mut on_cuboids = vec![intersection];
        on_cuboids.extend_from_slice(&remaining_0);
        on_cuboids.extend_from_slice(&remaining_1);

        let mut remaining_on: Vec<Box3D> = Vec::new();
        let ci3 = example.cuboid_instructions[2].box3d;
        for on_cuboid in on_cuboids {
            if let Some((_, rem_on, _)) = on_cuboid.split_intersecting(ci3) {
                remaining_on.extend_from_slice(&rem_on);
            } else {
                remaining_on.push(on_cuboid);
            }
        }
        let size_remaining_on: u64 = remaining_on.iter().filter_map(|b| b.size()).sum();
        assert_eq!(size_remaining_on, 38);

        // check fourth instruction
        let ci4 = example.cuboid_instructions[3].box3d;
        // expecting no intersection with remaining on blocks
        for on_cuboid in remaining_on.iter() {
            assert!(on_cuboid.split_intersecting(ci4).is_none());
        }
        remaining_on.push(ci4);

        let size_remaining_on: u64 = remaining_on.iter().filter_map(|b| b.size()).sum();
        assert_eq!(size_remaining_on, 39);

        let result_part1 = example.solution_part_1();
        println!("result day_22 part 1: {result_part1}");
        assert_eq!(result_part1, 39);

        Ok(())
    }

    #[test]
    fn test_example_2_day_22() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_22_example_2.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_22 part 1: {result_part1}");
        assert_eq!(result_part1, 590_784);

        Ok(())
    }

    #[test]
    fn test_example_3_day_22() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_22_example_3.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_22 part 1: {result_part1}");
        assert_eq!(result_part1, 474_140);

        let result_part2 = example.solution_part_2();
        println!("result day_22 part 2: {result_part2}");
        assert_eq!(result_part2, 2_758_514_936_282_235);

        Ok(())
    }
}
