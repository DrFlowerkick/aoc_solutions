//!day_08.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Day08Data {
    antennas: HashMap<char, Vec<Point>>,
    max_x: i64,
    max_y: i64,
}

impl From<&str> for Day08Data {
    fn from(value: &str) -> Self {
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        let mut max_x: i64 = 0;
        let mut max_y: i64 = 0;
        for (y, line) in value.lines().enumerate().map(|(i, c)| (i as i64, c)) {
            max_y = max_y.max(y);
            for (x, label) in line.chars().enumerate().map(|(i, c)| (i as i64, c)) {
                max_x = max_x.max(x);
                if label != '.' {
                    let point = Point::new(x, y);
                    antennas
                        .entry(label)
                        .and_modify(|a| a.push(point))
                        .or_insert(vec![point]);
                }
            }
        }
        Self {
            antennas,
            max_x,
            max_y,
        }
    }
}

impl Day08Data {
    fn count_anti_nodes(&self) -> HashSet<Point> {
        let mut anti_nodes: HashSet<Point> = HashSet::new();
        for (_, points) in self.antennas.iter() {
            for (index, antenna_1) in points.iter().enumerate() {
                for antenna_2 in &points[index + 1..] {
                    let distance: Point = (
                        antenna_1.distance_x(*antenna_2),
                        antenna_1.distance_y(*antenna_2),
                    )
                        .into();
                    let anti_node = antenna_1.add(distance);
                    self.check_anti_node(anti_node, &mut anti_nodes);
                    let anti_node = antenna_2.subtract(distance);
                    self.check_anti_node(anti_node, &mut anti_nodes);
                }
            }
        }
        anti_nodes
    }
    fn check_anti_node(&self, anti_node: Point, anti_nodes: &mut HashSet<Point>) -> bool {
        if anti_node.x >= 0
            && anti_node.x <= self.max_x
            && anti_node.y >= 0
            && anti_node.y <= self.max_y
        {
            anti_nodes.insert(anti_node);
            true
        } else {
            false
        }
    }
    fn count_anti_nodes_extended(&self) -> HashSet<Point> {
        let mut anti_nodes: HashSet<Point> = HashSet::new();
        for (_, points) in self.antennas.iter() {
            for (index, antenna_1) in points.iter().enumerate() {
                for antenna_2 in &points[index + 1..] {
                    let distance: Point = (
                        antenna_1.distance_x(*antenna_2),
                        antenna_1.distance_y(*antenna_2),
                    )
                        .into();
                    self.extend_nodes_add(*antenna_2, distance, &mut anti_nodes);
                    self.extend_nodes_subtract(*antenna_1, distance, &mut anti_nodes);
                }
            }
        }
        anti_nodes
    }
    fn extend_nodes_add(
        &self,
        start_antenna: Point,
        distance: Point,
        anti_nodes: &mut HashSet<Point>,
    ) {
        let anti_node = start_antenna.add(distance);
        if self.check_anti_node(anti_node, anti_nodes) {
            self.extend_nodes_add(anti_node, distance, anti_nodes);
        }
    }
    fn extend_nodes_subtract(
        &self,
        start_antenna: Point,
        distance: Point,
        anti_nodes: &mut HashSet<Point>,
    ) {
        let anti_node = start_antenna.subtract(distance);
        if self.check_anti_node(anti_node, anti_nodes) {
            self.extend_nodes_subtract(anti_node, distance, anti_nodes);
        }
    }
}

pub fn day_08() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_08.txt");
    let challenge = Day08Data::from(input);

    let result_part1 = challenge.count_anti_nodes().len();
    println!("result day 08 part 1: {}", result_part1);
    assert_eq!(result_part1, 361);

    let result_part2 = challenge.count_anti_nodes_extended().len();
    println!("result day 08 part 2: {}", result_part2);
    assert_eq!(result_part2, 1_249);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use my_lib::my_map_point::MapPoint;
    use my_lib::my_map_two_dim::MyMap2D;

    const N: usize = 12;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_08_example.txt");
        let challenge = Day08Data::from(input);

        let result_part1 = challenge.count_anti_nodes();
        println!("result day 08 part 1: {}", result_part1.len());
        assert_eq!(result_part1.len(), 14);

        let result_part2 = challenge.count_anti_nodes_extended();
        println!("result day 08 part 2: {}", result_part2.len());
        let mut debug_map: MyMap2D<char, N, N> = MyMap2D::from(input);
        for point in result_part2.iter() {
            let map_point = MapPoint::<N, N>::new(point.x as usize, point.y as usize);
            let mp_value = debug_map.get_mut(map_point);
            if *mp_value == '.' {
                *mp_value = '#';
            }
        }
        println!("{}", debug_map);
        assert_eq!(result_part2.len(), 34);

        Ok(())
    }
}
