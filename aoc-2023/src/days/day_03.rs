//!day_03.rs

use anyhow::Result;
use my_lib::my_map_point::MapPoint;
use my_lib::my_map_two_dim::MyMap2D;

// number of chars in one line of day_03.txt
const X: usize = 140;
// number of lines in day_03.txt
const Y: usize = 140;

#[derive(Copy, Clone, Default)]
struct Cell {
    val: char,
    id: u32,
}

pub fn day_03() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_03.txt");
    let mut char_map: MyMap2D<Cell, X, Y> = MyMap2D::default();
    let mut id = 1;
    let mut last_is_digit = false;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if x == 0 && last_is_digit {
                id += 1;
                last_is_digit = false;
            }
            let cell = char_map.get_mut(MapPoint::<X, Y>::new(x, y));
            cell.val = c;
            if c.is_ascii_digit() {
                cell.id = id;
                last_is_digit = true;
            } else if last_is_digit {
                id += 1;
                last_is_digit = false;
            }
        }
    }

    let mut result_part1 = 0;
    let mut result_part2 = 0;
    last_is_digit = false;
    let mut is_part_number = false;
    let mut digits = String::new();
    for (point, cell) in char_map.iter() {
        // init new line
        if point.x() == 0 {
            if is_part_number {
                result_part1 += digits.parse::<u32>()?;
            }
            is_part_number = false;
            last_is_digit = false;
            digits = "".into();
        }

        if cell.val.is_ascii_digit() {
            digits.push(cell.val);
            last_is_digit = true;
            // check for is_part_number
            is_part_number = is_part_number
                || char_map
                    .iter_neighbors_with_corners(point)
                    .any(|(_, _, c)| c.val != '.' && !c.val.is_ascii_digit());
        } else {
            if is_part_number {
                result_part1 += digits.parse::<u32>()?;
                is_part_number = false;
            }
            if last_is_digit {
                digits = "".into();
                last_is_digit = false;
            }
        }
        // Part 2
        if cell.val == '*' {
            let mut ids: Vec<u32> = char_map
                .iter_neighbors_with_corners(point)
                .filter(|(_, _, c)| c.id > 0)
                .map(|(_, _, c)| c.id)
                .collect();
            ids.sort();
            ids.dedup();
            if ids.len() == 2 {
                let digits_id0 = String::from_iter(
                    char_map
                        .iter()
                        .filter(|(_, c)| c.id == ids[0])
                        .map(|(_, c)| c.val),
                )
                .parse::<u32>()?;
                let digits_id1 = String::from_iter(
                    char_map
                        .iter()
                        .filter(|(_, c)| c.id == ids[1])
                        .map(|(_, c)| c.val),
                )
                .parse::<u32>()?;
                result_part2 += digits_id0 * digits_id1;
            }
        }
    }

    println!("result day 03 part 1: {}", result_part1);
    assert_eq!(result_part1, 535_235);
    println!("result day 03 part 2: {}", result_part2);
    assert_eq!(result_part2, 79_844_424);
    Ok(())
}
