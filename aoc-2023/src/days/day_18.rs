//!day_18.rs

use anyhow::{Result, anyhow};

// https://www.youtube.com/watch?v=bGWK76_e-LM
// https://de.wikipedia.org/wiki/Gau%C3%9Fsche_Trapezformel
// https://de.wikipedia.org/wiki/Satz_von_Pick

#[derive(Default)]
struct LavaLagoon {
    points_task_1: Vec<(i64, i64)>,
    boundary_points_task1: i64,
    points_task_2: Vec<(i64, i64)>,
    boundary_points_task2: i64,
}

impl LavaLagoon {
    fn from_input(input: &str) -> Result<Self> {
        let mut lava_lagoon = Self::default();
        let mut cp_task1 = (0, 0);
        let mut cp_task2 = (0, 0);
        for line in input.lines() {
            let mut split_line = line.split_whitespace();
            let dir_task1: (i64, i64) = match split_line.next() {
                Some(d) => match d {
                    "R" => (1, 0),
                    "L" => (-1, 0),
                    "U" => (0, -1),
                    "D" => (0, 1),
                    _ => return Err(anyhow!("bad direction task 1 input")),
                },
                None => return Err(anyhow!("missing direction task 1 input")),
            };
            let ns_taks1 = match split_line.next() {
                Some(ns) => ns.parse::<i64>()?,
                None => return Err(anyhow!("missing number of steps task 1 input")),
            };
            cp_task1 = (
                cp_task1.0 + ns_taks1 * dir_task1.0,
                cp_task1.1 + ns_taks1 * dir_task1.1,
            );
            // boundary points is sum of steps
            lava_lagoon.boundary_points_task1 += ns_taks1;
            lava_lagoon.points_task_1.push(cp_task1);
            let (ns_taks2, dir_task2) = match split_line.next() {
                Some(rgb) => {
                    let ns_taks2 = &rgb[2..7];
                    let dir_task2 = match &rgb[7..8] {
                        "0" => (1, 0),  // R
                        "2" => (-1, 0), // L
                        "3" => (0, -1), // U
                        "1" => (0, 1),  // D
                        _ => return Err(anyhow!("bad direction task 2 input")),
                    };
                    (i64::from_str_radix(ns_taks2, 16)?, dir_task2)
                }
                None => return Err(anyhow!("missing rgb input")),
            };
            cp_task2 = (
                cp_task2.0 + ns_taks2 * dir_task2.0,
                cp_task2.1 + ns_taks2 * dir_task2.1,
            );
            // boundary points is sum of steps
            lava_lagoon.boundary_points_task2 += ns_taks2;
            lava_lagoon.points_task_2.push(cp_task2);
        }
        assert_eq!(cp_task1, (0, 0));
        assert_eq!(cp_task2, (0, 0));
        Ok(lava_lagoon)
    }
    fn calc_cubics(&self, task2: bool) -> i64 {
        let (points, boundary_points) = if task2 {
            (&self.points_task_2, self.boundary_points_task2)
        } else {
            (&self.points_task_1, self.boundary_points_task1)
        };
        // https://de.wikipedia.org/wiki/Gau%C3%9Fsche_Trapezformel
        let gauss_area = (points
            .iter()
            .enumerate()
            .map(|(index, (_, y))| {
                y * (points[self.decrement_index(index)].0 - points[self.increment_index(index)].0)
            })
            .sum::<i64>()
            / 2)
        .abs();
        // https://de.wikipedia.org/wiki/Satz_von_Pick
        let interiour_cubes = gauss_area - boundary_points / 2 + 1;
        interiour_cubes + boundary_points
    }
    fn increment_index(&self, index: usize) -> usize {
        if index == self.points_task_1.len() - 1 {
            return 0;
        }
        index + 1
    }
    fn decrement_index(&self, index: usize) -> usize {
        if index == 0 {
            return self.points_task_1.len() - 1;
        }
        index - 1
    }
}

pub fn day_18() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_18.txt");
    let lava_lagoon = LavaLagoon::from_input(input)?;
    let cubics_task1 = lava_lagoon.calc_cubics(false);
    println!("result day 18 part 1: {}", cubics_task1);
    assert_eq!(cubics_task1, 42_317);
    let cubics_task2 = lava_lagoon.calc_cubics(true);
    println!("result day 18 part 2: {}", cubics_task2);
    assert_eq!(cubics_task2, 83_605_563_360_288);
    Ok(())
}

#[cfg(test)]
mod tests {

    //use super::*;

    #[test]
    fn test_hexa_convert() {
        let hex1 = "70c71";
        let hex1_num = u32::from_str_radix(hex1, 16).unwrap();
        eprintln!("x{}: d{}", hex1, hex1_num);
    }
}
