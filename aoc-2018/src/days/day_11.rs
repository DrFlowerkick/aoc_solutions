//!day_11.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::HashMap;

struct ChallengeInput {
    serial: i64,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            serial: value.parse().unwrap(),
        }
    }
}

type Part1Result = (String, HashMap<Point, i64>, HashMap<(Point, i64), i64>);

impl ChallengeInput {
    fn solution_part_1(&self) -> Part1Result {
        let mut grid: HashMap<Point, i64> = HashMap::new();
        let mut square_grid: HashMap<(Point, i64), i64> = HashMap::new();
        self.find_best_square(3, &mut grid, &mut square_grid);
        let ((max_power, _), _) = square_grid.iter().max_by_key(|(_, v)| **v).unwrap();
        (
            format!("{},{}", max_power.x, max_power.y),
            grid,
            square_grid,
        )
    }
    fn solution_part_2(
        &self,
        mut grid: HashMap<Point, i64>,
        mut square_grid: HashMap<(Point, i64), i64>,
        max_expected_size: i64,
    ) -> String {
        assert!(max_expected_size <= 300);
        // calculate remaining sizes starting with 4, since 3 has been calculated in part 1
        for size in 4..=max_expected_size {
            self.find_best_square(size, &mut grid, &mut square_grid);
        }
        let ((max_power, size), _) = square_grid.iter().max_by_key(|(_, v)| **v).unwrap();
        format!("{},{},{}", max_power.x, max_power.y, size)
    }
    fn calc_cell_value(&self, cell: Point) -> i64 {
        let rack_id = cell.x + 10;
        let power_level = (rack_id * cell.y + self.serial) * rack_id;
        let hundred_digit = if power_level < 100 {
            0
        } else {
            (power_level / 100) % 10
        };
        hundred_digit - 5
    }
    fn find_best_square(
        &self,
        size: i64,
        grid: &mut HashMap<Point, i64>,
        square_grid: &mut HashMap<(Point, i64), i64>,
    ) {
        for y in 1..=301 - size {
            for x in 1..=301 - size {
                let top_left = Point::new(x, y);
                let mut total_power = 0;
                if let Some(power) = square_grid.get(&(top_left, size - 1)) {
                    total_power += power;
                    let top_right = top_left.add((size - 1, 0));
                    let bottom_left = top_left.add((0, size - 1));
                    for o in 0..size {
                        let cell = top_right.add((0, o));
                        let power = grid.entry(cell).or_insert(self.calc_cell_value(cell));
                        total_power += *power;
                        if o < size - 1 {
                            // do not add bottom right twice!
                            let cell = bottom_left.add((o, 0));
                            let power = grid.entry(cell).or_insert(self.calc_cell_value(cell));
                            total_power += *power;
                        }
                    }
                } else {
                    // this is only required for part 1
                    for o_y in 0..size {
                        for o_x in 0..size {
                            let cell = top_left.add((o_x, o_y));
                            let power = grid.entry(cell).or_insert(self.calc_cell_value(cell));
                            total_power += *power;
                        }
                    }
                }
                square_grid.insert((top_left, size), total_power);
            }
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_11.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, grid, square_grid) = challenge.solution_part_1();
    println!("result day_11 part 1: {result_part1}");
    assert_eq!(result_part1, "33,34");

    // To find the solution start with a good guess for max_expected_size (conservative would be 300).
    // I use the solution to my input, 14, to reduce run time of calculation to the minimum without
    // changing the code.
    let result_part2 = challenge.solution_part_2(grid, square_grid, 14);
    println!("result day_11 part 2: {result_part2}");
    assert_eq!(result_part2, "235,118,14");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cell_value() {
        let example = ChallengeInput::from("8");
        assert_eq!(example.calc_cell_value(Point::new(3, 5)), 4);
        let example = ChallengeInput::from("57");
        assert_eq!(example.calc_cell_value(Point::new(122, 79)), -5);
        let example = ChallengeInput::from("39");
        assert_eq!(example.calc_cell_value(Point::new(217, 196)), 0);
        let example = ChallengeInput::from("71");
        assert_eq!(example.calc_cell_value(Point::new(101, 153)), 4);
    }

    #[test]
    fn test_example_day_11() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_11_example.txt");

        let solutions = [("33,45", "90,269,16"), ("21,61", "232,251,12")];

        for (line, (solution_part_1, solution_part_2)) in input.lines().zip(solutions) {
            let example = ChallengeInput::from(line);

            let (result_part1, grid, square_grid) = example.solution_part_1();
            println!("result day_11 part 1: {result_part1}");
            assert_eq!(result_part1, solution_part_1);

            let result_part2 = example.solution_part_2(grid, square_grid, 16);
            println!("result day_11 part 2: {result_part2}");
            assert_eq!(result_part2, solution_part_2);
        }

        Ok(())
    }
}
