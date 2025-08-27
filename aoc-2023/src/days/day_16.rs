//!day_16.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};

// values taken from ../../../../aoc_input/aoc-2023/day_16.txt
const X: usize = 110;
const Y: usize = 110;

#[derive(Default, Clone, Copy)]
enum CellType {
    #[default]
    Empty,
    MirrorBottomLeftTopRight,
    MirrorTopLeftBottomRight,
    SplitterVertical,
    SplitterHorizontal,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '.' => CellType::Empty,
            '/' => CellType::MirrorBottomLeftTopRight,
            '\\' => CellType::MirrorTopLeftBottomRight,
            '|' => CellType::SplitterVertical,
            '-' => CellType::SplitterHorizontal,
            _ => panic!("Bad cell char"),
        }
    }
}

impl CellType {
    fn beam_movement(&self, beam_direction: Compass) -> (Compass, Option<Compass>) {
        // sanity check of beam_direction
        if !beam_direction.is_cardinal() {
            panic!("internal error beam direction");
        }

        // beam_direction points toward cell
        match self {
            CellType::Empty => (beam_direction, None),
            CellType::MirrorBottomLeftTopRight => match beam_direction {
                Compass::N => (Compass::E, None),
                Compass::E => (Compass::N, None),
                Compass::S => (Compass::W, None),
                Compass::W => (Compass::S, None),
                _ => panic!("internal error beam direction"),
            },
            CellType::MirrorTopLeftBottomRight => match beam_direction {
                Compass::N => (Compass::W, None),
                Compass::E => (Compass::S, None),
                Compass::S => (Compass::E, None),
                Compass::W => (Compass::N, None),
                _ => panic!("internal error beam direction"),
            },
            CellType::SplitterVertical => match beam_direction {
                Compass::N | Compass::S => (beam_direction, None),
                Compass::E | Compass::W => (Compass::N, Some(Compass::S)),
                _ => panic!("internal error beam direction"),
            },
            CellType::SplitterHorizontal => match beam_direction {
                Compass::E | Compass::W => (beam_direction, None),
                Compass::N | Compass::S => (Compass::E, Some(Compass::W)),
                _ => panic!("internal error beam direction"),
            },
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Cell {
    ctype: CellType,
    beam_counter: usize,
    beam_exit_n: bool,
    beam_exit_e: bool,
    beam_exit_s: bool,
    beam_exit_w: bool,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Cell {
            ctype: CellType::from(value),
            ..Default::default()
        }
    }
}

impl Cell {
    fn beam_movement(&mut self, beam_direction: Compass) -> (Option<Compass>, Option<Compass>) {
        // since a beam enters cell, increment beam_counter
        self.beam_counter += 1;

        // beam_direction describes from which direction the beam enters cell
        let (beam_1, beam_2) = self.ctype.beam_movement(beam_direction);
        let beam_1 = self.beam_check(beam_1);
        let beam_2 = match beam_2 {
            Some(beam) => self.beam_check(beam),
            None => None,
        };
        (beam_1, beam_2)
    }

    fn beam_check(&mut self, beam_direction: Compass) -> Option<Compass> {
        let beam_check = match beam_direction {
            Compass::N => &mut self.beam_exit_n,
            Compass::E => &mut self.beam_exit_e,
            Compass::S => &mut self.beam_exit_s,
            Compass::W => &mut self.beam_exit_w,
            _ => panic!("internal error beam direction"),
        };

        if *beam_check {
            None
        } else {
            *beam_check = true;
            Some(beam_direction)
        }
    }
}

#[derive(Default)]
struct MirrorChamber<const X: usize, const Y: usize> {
    map: MyMap2D<Cell, X, Y>,
}

impl<const X: usize, const Y: usize> From<&str> for MirrorChamber<X, Y> {
    fn from(value: &str) -> Self {
        MirrorChamber {
            map: MyMap2D::from(value),
        }
    }
}

impl<const X: usize, const Y: usize> MirrorChamber<X, Y> {
    fn part1_beam_movement(&mut self) -> usize {
        self.beam_movement((0, 0).into(), Compass::E);
        self.energized_cells()
    }
    fn beam_movement(&mut self, current_cell: MapPoint<X, Y>, beam_direction: Compass) {
        let (beam_1, beam_2) = self.map.get_mut(current_cell).beam_movement(beam_direction);
        if let Some(beam_1_direction) = beam_1 {
            if let Some(next_cell) = current_cell.neighbor(beam_1_direction) {
                self.beam_movement(next_cell, beam_1_direction);
            }
        }
        if let Some(beam_2_direction) = beam_2 {
            if let Some(next_cell) = current_cell.neighbor(beam_2_direction) {
                self.beam_movement(next_cell, beam_2_direction);
            }
        }
    }
    fn energized_cells(&self) -> usize {
        self.map.iter().filter(|(_, c)| c.beam_counter > 0).count()
    }
    fn part2_beam_movement(&mut self) -> usize {
        let mut max_energy = 0;
        for point in MapPoint::<X, Y>::new(0, 0).iter_edge(false) {
            self.reset_beam_data();
            match point.map_position() {
                Compass::NE | Compass::SE | Compass::SW | Compass::NW => {
                    // corner -> check from two directions
                    self.beam_movement(point, point.map_position().clockwise().flip());
                    max_energy = max_energy.max(self.energized_cells());
                    self.reset_beam_data();
                    self.beam_movement(point, point.map_position().counterclockwise().flip());
                    max_energy = max_energy.max(self.energized_cells());
                }
                Compass::N | Compass::E | Compass::S | Compass::W => {
                    self.beam_movement(point, point.map_position().flip());
                    max_energy = max_energy.max(self.energized_cells());
                }
                Compass::Center => (),
            }
        }
        max_energy
    }
    fn reset_beam_data(&mut self) {
        for cell in self.map.iter_mut().map(|(_, c)| c) {
            cell.beam_counter = 0;
            cell.beam_exit_n = false;
            cell.beam_exit_e = false;
            cell.beam_exit_s = false;
            cell.beam_exit_w = false;
        }
    }
}

pub fn day_16() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_16.txt");
    let mut mirror_chamber = MirrorChamber::<X, Y>::from(input);
    let result_part1 = mirror_chamber.part1_beam_movement();
    println!("result day 16 part 1: {}", result_part1);
    assert_eq!(result_part1, 7_498);
    let result_part2 = mirror_chamber.part2_beam_movement();
    println!("result day 16 part 2: {}", result_part2);
    assert_eq!(result_part2, 7_846);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    const XT: usize = 10;
    const YT: usize = 10;

    #[test]
    fn test_part1_example() {
        let input = ".|...\\....\n\
                           |.-.\\.....\n\
                           .....|-...\n\
                           ........|.\n\
                           ..........\n\
                           .........\\\n\
                           ..../.\\\\..\n\
                           .-.-/..|..\n\
                           .|....-|.\\\n\
                           ..//.|....";
        let mut mirror_chamber = MirrorChamber::<XT, YT>::from(input);
        mirror_chamber.part1_beam_movement();
        let result_part1 = mirror_chamber
            .map
            .iter()
            .filter(|(_, c)| c.beam_counter > 0)
            .count();
        println!("result day 16 example part 1: {}", result_part1);
        assert_eq!(result_part1, 46);
    }
}
