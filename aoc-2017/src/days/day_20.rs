//!day_20.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point3D;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Particle {
    pos: Point3D,
    vel: Point3D,
    acc: Point3D,
}

impl From<&str> for Particle {
    fn from(value: &str) -> Self {
        let points: Vec<Point3D> = value
            .split(", ")
            .filter_map(|raw| raw.split_once("=<"))
            .filter_map(|(_, raw)| raw.strip_suffix(">"))
            .map(|raw| {
                let coordinates: Vec<i64> = raw.split(",").filter_map(|d| d.parse().ok()).collect();
                Point3D {
                    x: coordinates[0],
                    y: coordinates[1],
                    z: coordinates[2],
                }
            })
            .collect();
        Self {
            pos: points[0],
            vel: points[1],
            acc: points[2],
        }
    }
}

impl Particle {
    fn one_tick(&mut self) {
        self.vel = self.vel.add(self.acc);
        self.pos = self.pos.add(self.vel);
    }
}

struct ChallengeInput {
    particles: Vec<Particle>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            particles: value.lines().map(Particle::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> usize {
        let min_a = self
            .particles
            .iter()
            .map(|p| p.acc.delta(Point3D::default()))
            .min()
            .unwrap();
        let particles_with_min_a: Vec<Particle> = self
            .particles
            .iter()
            .filter(|p| p.acc.delta(Point3D::default()) == min_a)
            .copied()
            .collect();
        let min_v = particles_with_min_a
            .iter()
            .map(|p| p.vel.delta(Point3D::default()))
            .min()
            .unwrap();
        let particles_with_min_v: Vec<Particle> = particles_with_min_a
            .iter()
            .filter(|p| p.vel.delta(Point3D::default()) == min_v)
            .copied()
            .collect();
        assert_eq!(particles_with_min_v.len(), 1);
        self.particles
            .iter()
            .position(|p| *p == particles_with_min_v[0])
            .unwrap()
    }
    fn solution_part_2(&self) -> usize {
        let mut particles = self.particles.clone();
        let mut counter = 0;
        let max_count = 10;
        while counter < max_count {
            counter += 1;
            particles.iter_mut().for_each(|p| p.one_tick());

            let mut not_unique_pos: HashSet<Point3D> = HashSet::new();
            for (i, a) in particles.iter().enumerate() {
                for b in particles.iter().skip(i + 1) {
                    if a.pos == b.pos {
                        not_unique_pos.insert(a.pos);
                    }
                }
            }
            if !not_unique_pos.is_empty() {
                particles = particles
                    .iter()
                    .filter(|p| !not_unique_pos.contains(&p.pos))
                    .copied()
                    .collect();
                counter = 0;
            }
        }

        particles.len()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_20.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_20 part 1: {result_part1}");
    assert_eq!(result_part1, 457);

    let result_part2 = challenge.solution_part_2();
    println!("result day_20 part 2: {result_part2}");
    assert_eq!(result_part2, 448);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_20_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_20_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_20 part 1: {result_part1}");
        assert_eq!(result_part1, 0);

        Ok(())
    }

    #[test]
    fn test_example_day_20_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_20_example_part_2.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_20 part 2: {result_part2}");
        assert_eq!(result_part2, 1);

        Ok(())
    }
}
