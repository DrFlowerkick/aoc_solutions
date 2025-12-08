//!day_19.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::{Point3D, Turns90};
use std::collections::{HashMap, HashSet};

type RotationComb = (Turns90, Turns90, Turns90);
// quadratic distance, count
type Fingerprint = HashMap<i64, usize>;

struct ChallengeInput {
    beacons: Vec<Vec<Point3D>>,
    fingerprints: Vec<Fingerprint>,
    scanner_positions: Vec<Option<(Point3D, RotationComb)>>,
    min_identical_beacons: usize,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let beacons: Vec<Vec<Point3D>> = value
            .split("\n\n")
            .map(|scanner| {
                scanner
                    .lines()
                    .skip(1)
                    .map(|p| {
                        let numbers: Vec<i64> =
                            p.split(',').filter_map(|n| n.parse().ok()).collect();
                        Point3D::new(numbers[0], numbers[1], numbers[2])
                    })
                    .collect()
            })
            .collect();
        let mut scanner_positions: Vec<Option<(Point3D, RotationComb)>> = vec![None; beacons.len()];
        scanner_positions[0] = Some((
            Point3D::new(0, 0, 0),
            (Turns90::T0, Turns90::T0, Turns90::T0),
        ));
        ChallengeInput {
            beacons,
            scanner_positions,
            fingerprints: Vec::new(),
            min_identical_beacons: 12,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> usize {
        // calc fingerprints of all scanner beacons
        self.calc_fingerprints();
        // try to align
        while self.scanner_positions.iter().any(|sp| sp.is_none()) {
            for index_1 in 0..self.beacons.len() {
                if self.scanner_positions[index_1].is_none() {
                    continue;
                }
                for index_2 in 0..self.beacons.len() {
                    if self.scanner_positions[index_2].is_some() {
                        continue;
                    }
                    self.try_calc_scanner_position(index_1, index_2);
                }
            }
        }
        let all_beacons: HashSet<Point3D> = self
            .beacons
            .iter()
            .enumerate()
            .flat_map(|(index, b)| {
                let (scanner_position, rotation_combination) =
                    self.scanner_positions[index].unwrap();
                b.iter().map(move |beacon| {
                    beacon
                        .apply_rotation_combination(rotation_combination)
                        .add(scanner_position)
                })
            })
            .collect();
        all_beacons.len()
    }
    fn solution_part_2(&self) -> i64 {
        let mut max_distance = i64::MIN;
        for (index, (scanner_position_1, _)) in self
            .scanner_positions
            .iter()
            .filter_map(|sp| *sp)
            .enumerate()
        {
            for (scanner_position_2, _) in self
                .scanner_positions
                .iter()
                .filter_map(|sp| *sp)
                .skip(index + 1)
            {
                let delta = scanner_position_1.subtract(scanner_position_2);
                max_distance = max_distance.max(delta.x.abs() + delta.y.abs() + delta.z.abs());
            }
        }
        max_distance
    }
    fn calc_fingerprints(&mut self) {
        for scanner_index in 0..self.beacons.len() {
            let mut fingerprint: Fingerprint = HashMap::new();
            for (index, beacon_a) in self.beacons[scanner_index].iter().enumerate() {
                for beacon_b in self.beacons[scanner_index].iter().skip(index + 1) {
                    let delta = beacon_a.subtract(*beacon_b);
                    *fingerprint
                        .entry(delta.x * delta.x + delta.y * delta.y + delta.z * delta.z)
                        .or_default() += 1;
                }
            }
            self.fingerprints.push(fingerprint);
        }
    }
    fn try_calc_scanner_position(&mut self, index_1: usize, index_2: usize) {
        // check if fingerprint indicates overlapping sensor beacons:
        // they overlap, if at least (min_identical_beacons over 2) of quadratic
        // distances are identical in both fingerprints
        let num_identical_quadratic_distances: usize = self.fingerprints[index_1]
            .iter()
            .map(|(quadratic_distance, count)| {
                count.min(
                    self.fingerprints[index_2]
                        .get(quadratic_distance)
                        .unwrap_or(&0),
                )
            })
            .sum();
        // overlapping beacons in both sensor data sets share the same quadratic distances to each other
        // mathematically speaking; (m over 2) identical distances ar required (2 because connection of 2 points)
        // m: required beacons -> min_identical_beacons
        let overlap_threshold = self.min_identical_beacons * (self.min_identical_beacons - 1) / 2;
        if num_identical_quadratic_distances < overlap_threshold {
            // not enough overlapping
            return;
        }
        // enough overlapping -> find alignment
        let (scanner_position, scanner_rotation) = self.scanner_positions[index_1].unwrap();
        for rotation_combination in Point3D::all_unambiguous_rotation_combinations() {
            let mut translation_count: HashMap<Point3D, usize> = HashMap::new();
            for index_1_a in 0..self.beacons[index_1].len() {
                let point_a = self.beacons[index_1][index_1_a];
                // we want all points be in the same orientation of the first sensor set
                let point_a = point_a
                    .apply_rotation_combination(scanner_rotation)
                    .add(scanner_position);
                for index_2_b in 0..self.beacons[index_2].len() {
                    let point_b = self.beacons[index_2][index_2_b];
                    let point_b = point_b.apply_rotation_combination(rotation_combination);
                    let translation = point_a.subtract(point_b);
                    *translation_count.entry(translation).or_default() += 1;
                }
            }
            // at least min_identical_beacons of translations must be identical
            if let Some((translation, count)) = translation_count.iter().max_by_key(|(_, c)| *c)
                && *count >= self.min_identical_beacons
            {
                // found correct rotation
                self.scanner_positions[index_2] = Some((*translation, rotation_combination));
                return;
            }
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_19.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_19 part 1: {result_part1}");
    assert_eq!(result_part1, 394);

    let result_part2 = challenge.solution_part_2();
    println!("result day_19 part 2: {result_part2}");
    //assert_eq!(result_part2, YYY);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day_19() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_19_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_19 part 1: {result_part1}");
        assert_eq!(result_part1, 79);

        let result_part2 = example.solution_part_2();
        println!("result day_19 part 2: {result_part2}");
        assert_eq!(result_part2, 3621);

        Ok(())
    }
}
