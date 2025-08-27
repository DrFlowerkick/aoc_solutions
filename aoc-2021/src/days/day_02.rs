//!day_02.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;

fn dive(input: &str) -> i64 {
    let mut position = Point::new(0, 0);
    for (command, distance) in input
        .lines()
        .filter_map(|l| l.split_once(' '))
        .filter_map(|(c, d)| d.parse::<i64>().ok().map(|dis| (c, dis)))
    {
        match command {
            "forward" => position.x += distance,
            "down" => position.y += distance,
            "up" => {
                position.y -= distance;
                if position.y < 0 {
                    println!("Up into the air!!!");
                    position.y = 0; // Prevent negative depth
                }
            }
            _ => (),
        }
    }
    position.x * position.y
}

fn dive_with_aim(input: &str) -> i64 {
    let mut position = Point::new(0, 0);
    let mut aim = 0;
    for (command, distance) in input
        .lines()
        .filter_map(|l| l.split_once(' '))
        .filter_map(|(c, d)| d.parse::<i64>().ok().map(|dis| (c, dis)))
    {
        match command {
            "forward" => {
                position.x += distance;
                position.y += aim * distance;
                if position.y < 0 {
                    println!("Up into the air!!!");
                    position.y = 0; // Prevent negative depth
                }
            }
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => (),
        }
    }
    position.x * position.y
}

pub fn day_02() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_02.txt");

    let result_part1 = dive(input);
    println!("result day 02 part 1: {result_part1}");
    assert_eq!(result_part1, 2_117_664);

    let result_part2 = dive_with_aim(input);
    println!("result day 02 part 2: {result_part2}");
    assert_eq!(result_part2, 2_073_416_724);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_02_example.txt");

        let result_part1 = dive(input);
        println!("result day 02 part 1: {result_part1}");
        assert_eq!(result_part1, 150);

        let result_part2 = dive_with_aim(input);
        println!("result day 02 part 2: {result_part2}");
        assert_eq!(result_part2, 900);

        Ok(())
    }
}
