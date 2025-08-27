//!day_14.rs

use anyhow::{anyhow, Result};
use my_lib::{my_compass::Compass, my_map_two_dim::MyMap2D};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

// values for X and Y taken from ../../../../aoc_input/aoc-2023/day_14.txt
const X: usize = 100;
const Y: usize = 100;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
enum Cell {
    #[default]
    None,
    Cube,
    Round,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::None,
            '#' => Cell::Cube,
            'O' => Cell::Round,
            _ => panic!("bad Cell char"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::None => write!(f, "."),
            Cell::Cube => write!(f, "#"),
            Cell::Round => write!(f, "O"),
        }
    }
}

fn rotate_round_cells_left_until_cube_in_slice(slice: &mut [Cell]) {
    let mut start = 0;
    while let Some(new_start) = slice.iter().skip(start).position(|c| *c != Cell::Cube) {
        start += new_start;
        let end = match slice.iter().skip(start).position(|c| *c == Cell::Cube) {
            Some(new_end) => start + new_end,
            None => slice.len(),
        };
        let mut index = 0;
        let num_rounds = slice[start..end]
            .iter()
            .filter(|c| **c == Cell::Round)
            .count();
        while index < num_rounds {
            if slice[start + index..end][0] == Cell::None {
                slice[start + index..end].rotate_left(1);
            } else {
                index += 1;
            }
        }
        start = end;
    }
}

fn rotate_round_cells_right_until_cube_in_slice(slice: &mut [Cell]) {
    let mut start = 0;
    while let Some(new_start) = slice.iter().skip(start).position(|c| *c != Cell::Cube) {
        start += new_start;
        let end = match slice.iter().skip(start).position(|c| *c == Cell::Cube) {
            Some(new_end) => start + new_end,
            None => slice.len(),
        };
        let mut index = 0;
        let num_rounds = slice[start..end]
            .iter()
            .filter(|c| **c == Cell::Round)
            .count();
        while index < num_rounds {
            if slice[start..end - index][end - start - 1 - index] == Cell::None {
                slice[start..end - index].rotate_right(1);
            } else {
                index += 1;
            }
        }
        start = end;
    }
}

//impl Hash for MyMap2D<Cell, X, Y> {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Parabolic<const X: usize, const Y: usize> {
    platform: MyMap2D<Cell, X, Y>,
}

impl<const X: usize, const Y: usize> Parabolic<X, Y> {
    fn new(value: &str) -> Self {
        Parabolic {
            platform: value.into(),
        }
    }
    fn tilt_one_cycle(&mut self) -> Result<()> {
        self.tilt_direction(Compass::N)?;
        self.tilt_direction(Compass::W)?;
        self.tilt_direction(Compass::S)?;
        self.tilt_direction(Compass::E)
    }
    fn tilt_num_cycles(
        &mut self,
        cycles: usize,
        cache: &mut HashMap<Parabolic<X, Y>, Parabolic<X, Y>>,
    ) -> Result<()> {
        let mut found_cache_first_time: Option<Self> = None;
        let mut counter_first_time = 0;
        let mut found_pattern_cycle = false;
        let mut return_counter = 0;
        for counter in 1..=cycles {
            match cache.get(self) {
                Some(para_plat) => {
                    *self = *para_plat;
                    match found_cache_first_time {
                        Some(first) => {
                            if !found_pattern_cycle && first == *para_plat {
                                let pattern_cycle = counter - counter_first_time;
                                found_pattern_cycle = true;
                                return_counter = counter + (cycles - counter) % pattern_cycle;
                            } else if found_pattern_cycle && counter == return_counter {
                                return Ok(());
                            }
                        }
                        None => {
                            found_cache_first_time = Some(*para_plat);
                            counter_first_time = counter;
                        }
                    }
                }
                None => {
                    let pre_tilt = *self;
                    self.tilt_one_cycle()?;
                    cache.insert(pre_tilt, *self);
                }
            }
        }
        Ok(())
    }
    fn tilt_direction(&mut self, direction: Compass) -> Result<()> {
        match direction {
            Compass::N => {
                for col in 0..X {
                    let mut column = self.platform.get_column(col);
                    rotate_round_cells_left_until_cube_in_slice(&mut column[..]);
                    self.platform.apply_column(col, column);
                }
                Ok(())
            }
            Compass::W => {
                for row in 0..Y {
                    let row = self.platform.get_row_mut(row);
                    rotate_round_cells_left_until_cube_in_slice(row);
                }
                Ok(())
            }
            Compass::S => {
                for col in 0..X {
                    let mut column = self.platform.get_column(col);
                    rotate_round_cells_right_until_cube_in_slice(&mut column[..]);
                    self.platform.apply_column(col, column);
                }
                Ok(())
            }
            Compass::E => {
                for row in 0..Y {
                    let row = self.platform.get_row_mut(row);
                    rotate_round_cells_right_until_cube_in_slice(row);
                }
                Ok(())
            }
            _ => Err(anyhow!("direction not available")),
        }
    }
    fn calc_total_load_north(&self) -> usize {
        let mut total_load_north = 0;
        for row in 0..Y {
            total_load_north += (Y - row)
                * self
                    .platform
                    .iter_row(row)
                    .filter(|(_, c)| **c == Cell::Round)
                    .count();
        }
        total_load_north
    }
}

pub fn day_14() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_14.txt");
    let mut parabolic_platform: Parabolic<X, Y> = Parabolic::new(input);
    parabolic_platform.tilt_direction(Compass::N)?;
    let result_part1 = parabolic_platform.calc_total_load_north();
    println!("result day 14 part 1: {}", result_part1);
    assert_eq!(result_part1, 108_144);

    // part 2: reset platform
    let mut cache: HashMap<Parabolic<X, Y>, Parabolic<X, Y>> = HashMap::new();
    parabolic_platform = Parabolic::new(input);
    parabolic_platform.tilt_num_cycles(1_000_000_000, &mut cache)?;
    let result_part2 = parabolic_platform.calc_total_load_north();
    println!("result day 14 part 2: {}", result_part2);
    assert_eq!(result_part2, 108_404);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_slice_rotation_with_column() {
        let input = include_str!("../../../../aoc_input/aoc-2023/day_14.txt");
        let mut parabolic_platform: Parabolic<X, Y> = Parabolic::new(input);
        let mut col_0 = parabolic_platform.platform.get_column(0);
        eprintln!("before rotation: {:?}", col_0);
        let start = col_0.iter().position(|c| *c != Cell::Cube).unwrap();
        let end = start
            + col_0
                .iter()
                .skip(start)
                .position(|c| *c == Cell::Cube)
                .unwrap();
        let slice = &mut col_0[start..end];
        eprintln!("start: {}, end: {}", start, end);
        eprintln!("slice: {:?}", slice);
        slice.rotate_left(1);
        eprintln!("slice after rotation left 1: {:?}", slice);
        eprintln!("column after rotation left 1: {:?}", col_0);
        eprintln!("{}", parabolic_platform.platform);
        parabolic_platform.platform.apply_column(0, col_0);
        eprintln!(
            "parabolic_platform after slice rotation in col 0\n{}",
            parabolic_platform.platform
        );
    }

    #[test]
    fn test_tilting() {
        let input = include_str!("../../../../aoc_input/aoc-2023/day_14.txt");
        let mut parabolic_platform: Parabolic<X, Y> = Parabolic::new(input);
        let col_index = 20;
        // for column only N or S
        let direction = Compass::S;
        let mut col = String::new();
        for c in parabolic_platform.platform.get_column(col_index).iter() {
            col = format!("{}{}", col, c);
        }
        eprintln!("col {}:\n{}", col_index, col);
        parabolic_platform.tilt_direction(direction).unwrap();
        col = "".into();
        for c in parabolic_platform.platform.get_column(col_index).iter() {
            col = format!("{}{}", col, c);
        }
        eprintln!("col {} after tilt north\n{}", col_index, col);
    }

    #[test]
    fn test_ccyling() {
        const XT: usize = 10;
        const YT: usize = 10;
        let input = "O....#....\n\
                           O.OO#....#\n\
                           .....##...\n\
                           OO.#O....O\n\
                           .O.....O#.\n\
                           O.#..O.#.#\n\
                           ..O..#O..O\n\
                           .......O..\n\
                           #....###..\n\
                           #OO..#....";
        let mut parabolic_platform: Parabolic<XT, YT> = Parabolic::new(input);
        eprintln!("{}", parabolic_platform.platform);
        let one_cycle = ".....#....\n\
                               ....#...O#\n\
                               ...OO##...\n\
                               .OO#......\n\
                               .....OOO#.\n\
                               .O#...O#.#\n\
                               ....O#....\n\
                               ......OOOO\n\
                               #...O###..\n\
                               #..OO#....";
        let one_cycle: Parabolic<XT, YT> = Parabolic::new(one_cycle);
        parabolic_platform.tilt_one_cycle().unwrap();
        eprintln!("one cycle\n{}", parabolic_platform.platform);
        assert_eq!(one_cycle, parabolic_platform);
        let two_cycles = ".....#....\n\
                                ....#...O#\n\
                                .....##...\n\
                                ..O#......\n\
                                .....OOO#.\n\
                                .O#...O#.#\n\
                                ....O#...O\n\
                                .......OOO\n\
                                #..OO###..\n\
                                #.OOO#...O";
        let two_cycles: Parabolic<XT, YT> = Parabolic::new(two_cycles);
        parabolic_platform.tilt_one_cycle().unwrap();
        eprintln!("two cycle\n{}", parabolic_platform.platform);
        assert_eq!(two_cycles, parabolic_platform);
        let three_cycles = ".....#....\n\
                                  ....#...O#\n\
                                  .....##...\n\
                                  ..O#......\n\
                                  .....OOO#.\n\
                                  .O#...O#.#\n\
                                  ....O#...O\n\
                                  .......OOO\n\
                                  #...O###.O\n\
                                  #.OOO#...O";
        let three_cycles: Parabolic<XT, YT> = Parabolic::new(three_cycles);
        parabolic_platform.tilt_one_cycle().unwrap();
        eprintln!("three cycle\n{}", parabolic_platform.platform);
        assert_eq!(three_cycles, parabolic_platform);
        // test full cycling
        let mut cache: HashMap<Parabolic<XT, YT>, Parabolic<XT, YT>> = HashMap::new();
        parabolic_platform = Parabolic::new(input);
        parabolic_platform
            .tilt_num_cycles(1_000_000_000, &mut cache)
            .unwrap();
        assert_eq!(parabolic_platform.calc_total_load_north(), 64);
    }
}
