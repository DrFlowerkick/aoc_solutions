//!day_15.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Day15Data<const N: usize> {
    map: MyMap2D<char, N, N>,
    moves: Vec<Compass>,
    robot: MapPoint<N, N>,
}

impl<const N: usize> From<&str> for Day15Data<N> {
    fn from(value: &str) -> Self {
        let (map, moves) = value.split_once("\n\n").unwrap();
        let map = MyMap2D::from(map);
        let robot = map
            .iter()
            .find(|(_, c)| **c == '@')
            .map(|(r, _)| r)
            .unwrap();
        Self {
            map,
            moves: moves
                .chars()
                .filter_map(|c| match c {
                    '^' => Some(Compass::N),
                    '>' => Some(Compass::E),
                    'v' => Some(Compass::S),
                    '<' => Some(Compass::W),
                    _ => None,
                })
                .collect(),
            robot,
        }
    }
}

impl<const N: usize> Day15Data<N> {
    fn execute_robot_moves(&mut self) {
        for index in 0..self.moves.len() {
            let robot = self.robot;
            let direction = self.moves[index];
            self.move_object(robot, direction);
        }
    }
    fn move_object(&mut self, object: MapPoint<N, N>, direction: Compass) -> bool {
        let object_is_robot = self.robot == object;
        if let Some(neighbor) = object.neighbor(direction) {
            let neighbor_tile = *self.map.get(neighbor);
            match neighbor_tile {
                '#' => return false,
                '.' => {
                    self.map.swap_cell_values(object, neighbor);
                    if object_is_robot {
                        self.robot = neighbor;
                    }
                    return true;
                }
                'O' => {
                    if self.move_object(neighbor, direction) {
                        self.map.swap_cell_values(object, neighbor);
                        if object_is_robot {
                            self.robot = neighbor;
                        }
                        return true;
                    }
                }
                '@' => panic!("somehow we circled back to robot!"),
                _ => unreachable!("only certain symbols in map"),
            }
        }
        false
    }
    fn calc_gps_sum(&self) -> usize {
        self.map
            .iter()
            .filter(|(_, c)| **c == 'O')
            .map(|(p, _)| p.y() * 100 + p.x())
            .sum()
    }
}

#[derive(Debug)]
struct TwiceAsWide<const N: usize, const M: usize> {
    map: MyMap2D<char, M, N>,
    moves: Vec<Compass>,
    robot: MapPoint<M, N>,
}

impl<const N: usize, const M: usize> From<&Day15Data<N>> for TwiceAsWide<N, M> {
    fn from(value: &Day15Data<N>) -> Self {
        let mut map: MyMap2D<char, M, N> = MyMap2D::default();
        for (pnn, tnn) in value.map.iter() {
            let (pmn_1, pmn_2) = TwiceAsWide::<N, M>::pnn_to_pmn(pnn);
            match tnn {
                '#' => {
                    map.set(pmn_1, '#');
                    map.set(pmn_2, '#');
                }
                'O' => {
                    map.set(pmn_1, '[');
                    map.set(pmn_2, ']');
                }
                '.' => {
                    map.set(pmn_1, '.');
                    map.set(pmn_2, '.');
                }
                '@' => {
                    map.set(pmn_1, '@');
                    map.set(pmn_2, '.');
                }
                _ => unreachable!("only certain symbols in map"),
            }
        }
        let (robot, _) = TwiceAsWide::<N, M>::pnn_to_pmn(value.robot);
        Self {
            map,
            moves: value.moves.clone(),
            robot,
        }
    }
}

impl<const N: usize, const M: usize> TwiceAsWide<N, M> {
    fn pnn_to_pmn(pnn: MapPoint<N, N>) -> (MapPoint<M, N>, MapPoint<M, N>) {
        let pmn_1: MapPoint<M, N> = MapPoint::new(2 * pnn.x(), pnn.y());
        let pmn_2: MapPoint<M, N> = MapPoint::new(2 * pnn.x() + 1, pnn.y());
        (pmn_1, pmn_2)
    }
    fn execute_robot_moves(&mut self) {
        for index in 0..self.moves.len() {
            let direction = self.moves[index];
            self.move_robot(direction);
        }
    }
    fn move_robot(&mut self, direction: Compass) {
        let robot_moved = match direction {
            Compass::E | Compass::W => self.move_object_west_east(self.robot, &direction),
            Compass::N | Compass::S => {
                let mut objects: HashSet<MapPoint<M, N>> = HashSet::new();
                objects.insert(self.robot);
                self.move_object_north_south(objects, &direction)
            }
            _ => unreachable!("only certain directions of movement"),
        };

        if robot_moved {
            self.robot = self.robot.neighbor(direction).unwrap();
        }
    }
    fn move_object_north_south(
        &mut self,
        objects: HashSet<MapPoint<M, N>>,
        direction: &Compass,
    ) -> bool {
        let neighbors: HashMap<MapPoint<M, N>, MapPoint<M, N>> = objects
            .iter()
            .filter_map(|o| o.neighbor(*direction).map(|n| (n, *o)))
            .collect();
        if neighbors.keys().any(|n| *self.map.get(*n) == '#') {
            return false;
        }
        let is_free = neighbors.keys().all(|n| *self.map.get(*n) == '.');
        if is_free {
            for (neighbor, object) in neighbors.iter() {
                self.map.swap_cell_values(*object, *neighbor);
            }
            return true;
        }
        // get neighbors with objects and insert pairs of '[]'
        let mut object_neighbors: HashSet<MapPoint<M, N>> = HashSet::new();
        for object in neighbors.keys().filter(|n| *self.map.get(**n) != '.') {
            object_neighbors.insert(*object);
            if *self.map.get(*object) == '[' {
                object_neighbors.insert(object.neighbor(Compass::E).unwrap());
            }
            if *self.map.get(*object) == ']' {
                object_neighbors.insert(object.neighbor(Compass::W).unwrap());
            }
        }
        if self.move_object_north_south(object_neighbors, direction) {
            for (neighbor, object) in neighbors.iter() {
                self.map.swap_cell_values(*object, *neighbor);
            }
            return true;
        }
        false
    }
    fn move_object_west_east(&mut self, object: MapPoint<M, N>, direction: &Compass) -> bool {
        if let Some(neighbor) = object.neighbor(*direction) {
            let neighbor_tile = *self.map.get(neighbor);
            match neighbor_tile {
                '#' => return false,
                '.' => {
                    self.map.swap_cell_values(object, neighbor);
                    return true;
                }
                '[' | ']' => {
                    if self.move_object_west_east(neighbor, direction) {
                        self.map.swap_cell_values(object, neighbor);
                        return true;
                    }
                }
                '@' => panic!("somehow we circled back to robot!"),
                _ => unreachable!("only certain symbols in map"),
            }
        }
        false
    }
    fn calc_gps_sum(&self) -> usize {
        self.map
            .iter()
            .filter(|(_, c)| **c == '[')
            .map(|(p, _)| p.y() * 100 + p.x())
            .sum()
    }
}

// taken from ../../../../aoc_input/aoc-2024/day_15.txt
const N: usize = 50;
const M: usize = 100;

pub fn day_15() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_15.txt");
    let mut challenge = Day15Data::<N>::from(input);
    let mut twice_as_wide = TwiceAsWide::<N, M>::from(&challenge);
    challenge.execute_robot_moves();

    let result_part1 = challenge.calc_gps_sum();
    println!("result day 15 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_438_161);

    twice_as_wide.execute_robot_moves();
    let result_part2 = twice_as_wide.calc_gps_sum();
    println!("result day 15 part 2: {}", result_part2);
    assert_eq!(result_part2, 1_437_981);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // taken from ../../../../aoc_input/aoc-2024/day_15_example_01.txt
    const S: usize = 8;
    // taken from ../../../../aoc_input/aoc-2024/day_15_example_02.txt
    const B: usize = 10;
    const T: usize = 20;

    #[test]
    fn test_example_part() -> Result<()> {
        let example_01 = include_str!("../../../../aoc_input/aoc-2024/day_15_example_01.txt");
        let example_02 = include_str!("../../../../aoc_input/aoc-2024/day_15_example_02.txt");
        let mut challenge_01 = Day15Data::<S>::from(example_01);
        let mut challenge_02 = Day15Data::<B>::from(example_02);
        let mut twice_as_wide = TwiceAsWide::<B, T>::from(&challenge_02);

        challenge_01.execute_robot_moves();
        let result_part1_1 = challenge_01.calc_gps_sum();
        println!("result day 15 part 1, example 1: {}", result_part1_1);
        assert_eq!(result_part1_1, 2_028);

        challenge_02.execute_robot_moves();
        let result_part1_2 = challenge_02.calc_gps_sum();
        println!("result day 15 part 1, example 1: {}", result_part1_2);
        assert_eq!(result_part1_2, 10_092);

        let example_03 = include_str!("../../../../aoc_input/aoc-2024/day_15_example_03.txt");
        let example_03 = Day15Data::<7>::from(example_03);
        let mut example_03_twice_as_wide = TwiceAsWide::<7, 14>::from(&example_03);
        println!("{}", example_03_twice_as_wide.map);
        example_03_twice_as_wide.execute_robot_moves();
        println!("{}", example_03_twice_as_wide.map);

        println!("{}", twice_as_wide.map);
        twice_as_wide.execute_robot_moves();
        println!("{}", twice_as_wide.map);
        let result_part2 = twice_as_wide.calc_gps_sum();
        println!("result day 15 part 2: {}", result_part2);
        assert_eq!(result_part2, 9_021);

        Ok(())
    }
}
