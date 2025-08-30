//!day_17.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point3D;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point4D {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl From<(i64, i64, i64, i64)> for Point4D {
    fn from(value: (i64, i64, i64, i64)) -> Self {
        Point4D {
            x: value.0,
            y: value.1,
            z: value.2,
            w: value.3,
        }
    }
}

impl Point4D {
    fn add(&self, other: &Self) -> Self {
        Point4D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn iter_cuboid(
        &self,
        dx_minus: i64,
        dx_plus: i64,
        dy_minus: i64,
        dy_plus: i64,
        dz_minus: i64,
        dz_plus: i64,
        dw_minus: i64,
        dw_plus: i64,
    ) -> impl Iterator<Item = Point4D> + '_ {
        (dx_minus <= 0
            && dx_plus >= 0
            && dy_minus <= 0
            && dy_plus >= 0
            && dz_minus <= 0
            && dz_plus >= 0
            && dw_minus <= 0
            && dw_plus >= 0)
            .then(|| {
                (dw_minus..=dw_plus).flat_map(move |dw| {
                    (dz_minus..=dz_plus).flat_map(move |dz| {
                        (dy_minus..=dy_plus).flat_map(move |dy| {
                            (dx_minus..=dx_plus).map(move |dx| self.add(&(dx, dy, dz, dw).into()))
                        })
                    })
                })
            })
            .into_iter()
            .flatten()
    }
}

struct ChallengeInput {
    active_cubes: HashSet<Point3D>,
    ac_4d: HashSet<Point4D>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            active_cubes: value
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().filter_map(move |(x, c)| {
                        (c == '#').then_some((x as i64, y as i64, 0).into())
                    })
                })
                .collect(),
            ac_4d: value
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().filter_map(move |(x, c)| {
                        (c == '#').then_some((x as i64, y as i64, 0, 0).into())
                    })
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let mut current_cubes = self.active_cubes.clone();
        for _ in 0..6 {
            let mut new_cubes: HashSet<Point3D> = HashSet::new();
            let mut in_active_neighbor_cubes: HashSet<Point3D> = HashSet::new();
            // check active cubes
            for cube in current_cubes.iter() {
                let mut count_active = 0;
                for neighbor in cube.iter_cuboid(-1, 1, -1, 1, -1, 1) {
                    if neighbor == *cube {
                        continue;
                    }
                    if current_cubes.contains(&neighbor) {
                        count_active += 1;
                    } else {
                        in_active_neighbor_cubes.insert(neighbor);
                    }
                }
                if count_active == 2 || count_active == 3 {
                    new_cubes.insert(*cube);
                }
            }
            // check inactive neighbor cubes
            for cube in in_active_neighbor_cubes {
                let mut count_active = 0;
                for neighbor in cube.iter_cuboid(-1, 1, -1, 1, -1, 1) {
                    if neighbor == cube {
                        continue;
                    }
                    if current_cubes.contains(&neighbor) {
                        count_active += 1;
                    }
                }
                if count_active == 3 {
                    new_cubes.insert(cube);
                }
            }
            current_cubes = new_cubes;
        }
        current_cubes.len()
    }
    fn solution_part_2(&self) -> usize {
        let mut current_cubes = self.ac_4d.clone();
        for _ in 0..6 {
            let mut new_cubes: HashSet<Point4D> = HashSet::new();
            let mut in_active_neighbor_cubes: HashSet<Point4D> = HashSet::new();
            // check active cubes
            for cube in current_cubes.iter() {
                let mut count_active = 0;
                for neighbor in cube.iter_cuboid(-1, 1, -1, 1, -1, 1, -1, 1) {
                    if neighbor == *cube {
                        continue;
                    }
                    if current_cubes.contains(&neighbor) {
                        count_active += 1;
                    } else {
                        in_active_neighbor_cubes.insert(neighbor);
                    }
                }
                if count_active == 2 || count_active == 3 {
                    new_cubes.insert(*cube);
                }
            }
            // check inactive neighbor cubes
            for cube in in_active_neighbor_cubes {
                let mut count_active = 0;
                for neighbor in cube.iter_cuboid(-1, 1, -1, 1, -1, 1, -1, 1) {
                    if neighbor == cube {
                        continue;
                    }
                    if current_cubes.contains(&neighbor) {
                        count_active += 1;
                    }
                }
                if count_active == 3 {
                    new_cubes.insert(cube);
                }
            }
            current_cubes = new_cubes;
        }
        current_cubes.len()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_17.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_17 part 1: {result_part1}");
    assert_eq!(result_part1, 273);

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
        let input = include_str!("../../../../aoc_input/aoc-2020/day_17_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_17 part 1: {result_part1}");
        assert_eq!(result_part1, 112);

        let result_part2 = example.solution_part_2();
        println!("result day_17 part 2: {result_part2}");
        assert_eq!(result_part2, 848);

        Ok(())
    }
}
