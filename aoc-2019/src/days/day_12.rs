//!day_12.rs

use anyhow::Result;
use my_lib::my_algo_collection::gcd;
use std::cmp::Ordering;
use std::collections::HashSet;

// changes in one axis are independent from changes in other axis.
// therefore we can handle each axis separately, which is especially
// important for part two.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
struct MoonAxis {
    moons: [i64; 4],
    velocities: [i64; 4],
}

impl MoonAxis {
    fn set_initial_moon(&mut self, m: i64, index: usize) {
        self.moons[index] = m;
    }
    fn set_velocity(&mut self) {
        // compare pairwise positions and apply changes to velocity
        for (index_1, m_1) in self.moons.iter().enumerate() {
            for (index_2, m_2) in self.moons.iter().enumerate().skip(index_1 + 1) {
                match m_1.cmp(m_2) {
                    Ordering::Greater => {
                        self.velocities[index_1] -= 1;
                        self.velocities[index_2] += 1;
                    }
                    Ordering::Less => {
                        self.velocities[index_1] += 1;
                        self.velocities[index_2] -= 1;
                    }
                    Ordering::Equal => (), // equal position -> no change in velocity
                }
            }
        }
    }
    fn apply_velocity(&mut self) {
        self.moons
            .iter_mut()
            .zip(self.velocities.iter())
            .for_each(|(m, v)| *m += *v);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct ChallengeInput {
    x: MoonAxis,
    y: MoonAxis,
    z: MoonAxis,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut x = MoonAxis::default();
        let mut y = MoonAxis::default();
        let mut z = MoonAxis::default();
        for (index, axis) in value
            .lines()
            .map(|line| {
                let line = line.strip_prefix("<").unwrap();
                let line = line.strip_suffix(">").unwrap();
                line.split(", ")
                    .filter_map(|c| c.split_once("="))
                    .filter_map(|(_, c)| c.parse().ok())
                    .collect::<Vec<i64>>()
            })
            .enumerate()
        {
            x.set_initial_moon(axis[0], index);
            y.set_initial_moon(axis[1], index);
            z.set_initial_moon(axis[2], index);
        }
        ChallengeInput { x, y, z }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self, steps: usize) -> i64 {
        for _ in 0..steps {
            self.x.set_velocity();
            self.x.apply_velocity();
            self.y.set_velocity();
            self.y.apply_velocity();
            self.z.set_velocity();
            self.z.apply_velocity();
        }
        (0..4)
            .map(|index| {
                (self.x.moons[index].abs() + self.y.moons[index].abs() + self.z.moons[index].abs())
                    * (self.x.velocities[index].abs()
                        + self.y.velocities[index].abs()
                        + self.z.velocities[index].abs())
            })
            .sum()
    }
    fn solution_part_2(&mut self) -> i64 {
        let mut seen: HashSet<MoonAxis> = HashSet::new();
        // find cycle for each axis separately
        let mut cycle_x = 0;
        while seen.insert(self.x) {
            cycle_x += 1;
            self.x.set_velocity();
            self.x.apply_velocity();
        }

        seen.clear();
        let mut cycle_y = 0;
        while seen.insert(self.y) {
            cycle_y += 1;
            self.y.set_velocity();
            self.y.apply_velocity();
        }

        seen.clear();
        let mut cycle_z = 0;
        while seen.insert(self.z) {
            cycle_z += 1;
            self.z.set_velocity();
            self.z.apply_velocity();
        }
        // combine cycle times, respecting greatest common divider
        let gcd_x_y = gcd(cycle_x, cycle_y);
        let cycle_xy = cycle_x * cycle_y / gcd_x_y;

        let gcd_xy_z = gcd(cycle_xy, cycle_z);
        // cycle_xyz:
        cycle_xy * cycle_z / gcd_xy_z
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_12.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(1_000);
    println!("result day_12 part 1: {result_part1}");
    assert_eq!(result_part1, 12_351);

    // reset challenge
    let mut challenge = ChallengeInput::from(input);
    let result_part2 = challenge.solution_part_2();
    println!("result day_12 part 2: {result_part2}");
    assert_eq!(result_part2, 380_635_029_877_596);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_12() -> Result<()> {
        let multi_input = include_str!("../../../../aoc_input/aoc-2019/day_12_example.txt");
        let solutions = [(179, 2_772), (1_940, 4_686_774_924)];
        for (input, (solution_1, solution_2)) in multi_input.split("\n\n").zip(solutions) {
            let mut example = ChallengeInput::from(input);

            let result_part1 = if solution_1 == 179 {
                example.solution_part_1(10)
            } else {
                example.solution_part_1(100)
            };
            println!("result day_12 part 1: {result_part1}");
            assert_eq!(result_part1, solution_1);

            // reset example
            let mut example = ChallengeInput::from(input);
            let result_part2 = example.solution_part_2();
            println!("result day_12 part 2: {result_part2}");
            assert_eq!(result_part2, solution_2);
        }

        Ok(())
    }
}
