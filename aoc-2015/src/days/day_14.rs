//!day_14.rs

use anyhow::Result;

#[derive(Clone, Copy)]
struct Reindeer {
    speed: u64,
    run: u64,
    pause: u64,
}

impl From<&str> for Reindeer {
    fn from(value: &str) -> Self {
        let mut number_iter = value
            .split_whitespace()
            .filter_map(|n| n.parse::<u64>().ok());
        Reindeer {
            speed: number_iter.next().unwrap(),
            run: number_iter.next().unwrap(),
            pause: number_iter.next().unwrap(),
        }
    }
}

impl Reindeer {
    fn calc_distance(&self, time: u64) -> u64 {
        let cycles = time / (self.run + self.pause);
        let rem = time % (self.run + self.pause);
        let rem = rem.min(self.run);
        (cycles * self.run + rem) * self.speed
    }
}

struct ChallengeInput {
    reindeers: Vec<Reindeer>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            reindeers: value.lines().map(Reindeer::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self, time: u64) -> u64 {
        self.reindeers
            .iter()
            .map(|r| r.calc_distance(time))
            .max()
            .unwrap()
    }
    fn solution_part_2(&self, time: u64) -> u64 {
        let mut points: Vec<u64> = vec![0; self.reindeers.len()];
        for t in 1..=time {
            let distances: Vec<u64> = self.reindeers.iter().map(|r| r.calc_distance(t)).collect();
            let max = *distances.iter().max().unwrap();
            for (p, _) in points.iter_mut().zip(distances).filter(|(_, d)| *d == max) {
                *p += 1;
            }
        }
        points.into_iter().max().unwrap()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_14.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(2_503);
    println!("result day_14 part 1: {result_part1}");
    assert_eq!(result_part1, 2_640);

    let result_part2 = challenge.solution_part_2(2_503);
    println!("result day_14 part 2: {result_part2}");
    assert_eq!(result_part2, 1_102);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_14() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_14_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1(1_000);
        println!("result day_14 part 1: {result_part1}");
        assert_eq!(result_part1, 1_120);

        let result_part2 = example.solution_part_2(1000);
        println!("result day_14 part 2: {result_part2}");
        assert_eq!(result_part2, 689);

        Ok(())
    }
}
