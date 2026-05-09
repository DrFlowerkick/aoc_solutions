//!day_21.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::{Point, Turns90};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Square {
    square: Vec<Vec<char>>,
}

impl From<&str> for Square {
    fn from(value: &str) -> Self {
        Square {
            square: value.split("/").map(|l| l.chars().collect()).collect(),
        }
    }
}

impl Square {
    fn new(size: usize) -> Square {
        Square {
            square: vec![vec!['.'; size]; size],
        }
    }
    fn init_square() -> Square {
        Square {
            square: vec![
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ],
        }
    }
    fn size(&self) -> usize {
        self.square.len()
    }
    fn get_sub_square(&self, sub_size: usize, index: usize) -> Option<Square> {
        let size = self.size();
        let meta_size = size / sub_size;
        if !size.is_multiple_of(sub_size) || index >= meta_size * meta_size {
            return None;
        }
        let mut two_x_two = Square::new(2);
        let x = (index % meta_size) * sub_size;
        let y = (index / meta_size) * sub_size;
        for y_offset in 0..sub_size {
            for x_offset in 0..sub_size {
                two_x_two.square[y_offset][x_offset] = self.square[y + y_offset][x + x_offset];
            }
        }
        Some(two_x_two)
    }
    fn set_sub_square(&mut self, sub_size: usize, index: usize, sub_square: Square) {
        let size = self.size();
        let meta_size = size / sub_size;
        if !size.is_multiple_of(sub_size)
            || index >= meta_size * meta_size
            || sub_square.size() != sub_size
        {
            return;
        }
        let x = (index % meta_size) * sub_size;
        let y = (index / meta_size) * sub_size;
        for y_offset in 0..sub_size {
            for x_offset in 0..sub_size {
                self.square[y + y_offset][x + x_offset] = sub_square.square[y_offset][x_offset];
            }
        }
    }
    fn rotate(&self, turn: Turns90) -> Square {
        let points: HashMap<Point, char> = self
            .square
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .map(move |(x, c)| (Point::new(x as i64, y as i64), *c))
            })
            .collect();
        let mut point_map: HashMap<Point, Point> =
            points.keys().map(|p| (*p, p.turn(turn, true))).collect();
        let min_x = point_map.values().map(|p| p.x).min().unwrap();
        let min_y = point_map.values().map(|p| p.y).min().unwrap();
        let min = Point::new(min_x, min_y);
        // subtract min to move coordinates into 1. quadrant (only positive values for coordinates)
        point_map.iter_mut().for_each(|(_, v)| *v = v.subtract(min));
        let mut rotated = self.clone();
        for (rot, val) in point_map
            .iter()
            .map(|(orig, rot)| (rot, points.get(orig).unwrap()))
        {
            rotated.square[rot.y as usize][rot.x as usize] = *val;
        }
        rotated
    }
    fn mirror_x(&self) -> Square {
        let points: HashMap<Point, char> = self
            .square
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .map(move |(x, c)| (Point::new(x as i64, y as i64), *c))
            })
            .collect();
        let mut point_map: HashMap<Point, Point> =
            points.keys().map(|p| (*p, p.mirror_x())).collect();
        let min_x = point_map.values().map(|p| p.x).min().unwrap();
        let min_y = point_map.values().map(|p| p.y).min().unwrap();
        let min = Point::new(min_x, min_y);
        // subtract min to move coordinates into 1. quadrant (only positive values for coordinates)
        point_map.iter_mut().for_each(|(_, v)| *v = v.subtract(min));
        let mut mirror_x = self.clone();
        for (mir, val) in point_map
            .iter()
            .map(|(orig, mir)| (mir, points.get(orig).unwrap()))
        {
            mirror_x.square[mir.y as usize][mir.x as usize] = *val;
        }
        mirror_x
    }
    fn all_variations(&self) -> HashSet<Square> {
        let mirror_x = self.mirror_x();
        [Turns90::T0, Turns90::T90, Turns90::T180, Turns90::T270]
            .into_iter()
            .flat_map(|turn| [self.rotate(turn), mirror_x.rotate(turn)].into_iter())
            .collect()
    }
    fn count_on(&self) -> usize {
        self.square
            .iter()
            .flat_map(|l| l.iter())
            .filter(|c| **c == '#')
            .count()
    }
}

struct ChallengeInput {
    square_map: HashMap<Square, Square>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            square_map: value
                .lines()
                .map(|line| {
                    let (key, value) = line.split_once(" => ").unwrap();
                    let key = Square::from(key);
                    let value = Square::from(value);
                    (key, value)
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn fill_missing_patterns(&mut self) {
        let square_map = self.square_map.clone();
        for (key, value) in square_map.into_iter() {
            for variation_key in key.all_variations() {
                // insert if key does not exist
                self.square_map
                    .entry(variation_key)
                    .or_insert(value.clone());
            }
        }
    }
    fn solution_part_1(&self, iterations: usize) -> usize {
        let mut seen: HashMap<(Square, usize), usize> = HashMap::new();
        let init_square = Square::init_square();
        self.recursive_iteration(init_square, iterations, &mut seen)
    }
    fn solution_part_2(&self) -> usize {
        let mut seen: HashMap<(Square, usize), usize> = HashMap::new();
        let init_square = Square::init_square();
        self.recursive_iteration(init_square, 18, &mut seen)
    }
    fn recursive_iteration(
        &self,
        square: Square,
        iterations: usize,
        seen: &mut HashMap<(Square, usize), usize>,
    ) -> usize {
        if iterations == 0 {
            return square.count_on();
        }
        if let Some(count) = seen.get(&(square.clone(), iterations)) {
            return *count;
        }
        let count = match square.size() {
            3 => {
                // result is 4x4 square
                let next_square = self.square_map.get(&square).unwrap().clone();
                self.recursive_iteration(next_square, iterations - 1, seen)
            }
            4 => {
                let mut six_x_six = Square::new(6);
                for index in 0..4 {
                    let sub_square = square.get_sub_square(2, index).unwrap();
                    let next_square = self.square_map.get(&sub_square).unwrap().clone();
                    six_x_six.set_sub_square(3, index, next_square);
                }
                self.recursive_iteration(six_x_six, iterations - 1, seen)
            }
            6 => {
                // 6x6 square contains 9 2x2 squares. Each transforms into a 3x3 square
                // we sum up the count of each 3x3 square
                let mut count = 0;
                for index in 0..9 {
                    let sub_square = square.get_sub_square(2, index).unwrap();
                    let next_square = self.square_map.get(&sub_square).unwrap().clone();
                    count += self.recursive_iteration(next_square, iterations - 1, seen);
                }
                count
            }
            _ => panic!("unexpected size"),
        };

        seen.insert((square, iterations), count);
        count
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_21.txt");
    let mut challenge = ChallengeInput::from(input);
    challenge.fill_missing_patterns();

    let result_part1 = challenge.solution_part_1(5);
    println!("result day_21 part 1: {result_part1}");
    assert_eq!(result_part1, 171);

    let result_part2 = challenge.solution_part_2();
    println!("result day_21 part 2: {result_part2}");
    assert_eq!(result_part2, 2_498_142);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_21() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_21_example.txt");
        let mut example = ChallengeInput::from(input);
        example.fill_missing_patterns();

        let result_part1 = example.solution_part_1(2);
        println!("result day_21 part 1: {result_part1}");
        assert_eq!(result_part1, 12);

        Ok(())
    }
}
