//!day_06.rs

use anyhow::Result;

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
struct Population {
    // index is time to offspring, value at index is number of lantern fish with this timer
    timers: [u64; 9],
}

impl From<&str> for Population {
    fn from(input: &str) -> Self {
        let mut pop = Population { timers: [0; 9] };
        for index in input.split(',').filter_map(|i| i.parse::<usize>().ok()) {
            pop.timers[index] += 1;
        }

        pop
    }
}

impl Population {
    fn one_cycle(&mut self) {
        // lantern fish with timer zero will generate offspring with timer 8 and reset themselves to timer 6
        // --> rotate_left generates offspring at timer 8
        // adding number of lantern fish with timer 0 to timer 6 after rotate_left resets timer parents
        let num_zero = self.timers[0];
        self.timers.rotate_left(1);
        self.timers[6] += num_zero;
    }
    fn cycle(&mut self, days: usize) {
        for _ in 0..days {
            self.one_cycle();
        }
    }
    fn size(&self) -> u64 {
        self.timers.iter().sum()
    }
}

pub fn day_06() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_06.txt");
    let mut lantern_fish_population = Population::from(input);

    lantern_fish_population.cycle(80);
    let result_part1 = lantern_fish_population.size();
    println!("result day_06 part 1: {result_part1}");
    assert_eq!(result_part1, 380_243);

    lantern_fish_population.cycle(256 - 80);
    let result_part2 = lantern_fish_population.size();
    println!("result day_06 part 2: {result_part2}");
    assert_eq!(result_part2, 1_708_791_884_591);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_06_example.txt");
        let mut lantern_fish_population = Population::from(input);

        lantern_fish_population.cycle(80);
        let result_part1 = lantern_fish_population.size();
        println!("result day_06 part 1: {result_part1}");
        assert_eq!(result_part1, 5_934);

        lantern_fish_population.cycle(256 - 80);
        let result_part2 = lantern_fish_population.size();
        println!("result day_06 part 2: {result_part2}");
        assert_eq!(result_part2, 26_984_457_539);

        Ok(())
    }
}
