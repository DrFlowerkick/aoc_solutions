//!day_13.rs

use anyhow::Result;
use my_lib::my_algo_collection::{egcd, gcd};

struct ChallengeInput {
    my_timestamp: i64,
    busses: Vec<(i64, i64)>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (my_timestamp, busses) = value.split_once('\n').unwrap();
        let my_timestamp = my_timestamp.parse().unwrap();
        let busses = busses
            .split(',')
            .enumerate()
            .filter_map(|(index, b)| b.parse::<i64>().ok().map(|bus| (bus, index as i64)))
            .collect();
        ChallengeInput {
            my_timestamp,
            busses,
        }
    }
}

// situation: (t * m1 + r1) mod m2 = r2
// t is unknown
// calc_reminder returns r_t with:
// t = r_t + k * m1
// for every integer k we can calculate a value for t, which solves the situation above
fn calc_reminder(m1: i64, m2: i64, r1: i64, r2: i64) -> i64 {
    if gcd(m1, m2) != 1 {
        panic!("no solution possible");
    }
    let (_, x, _) = egcd(m1, m2);
    // make sure, inverse is positive
    let inverse_m1_mod_m2 = x.rem_euclid(m2);
    ((r2 - r1).rem_euclid(m2) * inverse_m1_mod_m2).rem_euclid(m2)
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let (bus, bus_timestamp) = self
            .busses
            .iter()
            .map(|&(bus, _)| (bus, bus * (1 + (self.my_timestamp / bus))))
            .min_by_key(|(_, bts)| *bts)
            .unwrap();
        (bus_timestamp - self.my_timestamp) * bus
    }
    fn solution_part_2(&self) -> i64 {
        // ts = a * b0
        // ts + 1 = b * b1
        // a * b0 + 1 = b * b1
        // b = (a * b0 + 1) / b1
        // b must be integer --> (a * b0 + 1) mod b1 = 0
        // find for each bus an a0, which is the first solution for above formula
        let bus_rem = self.transform_to_reminder_form();
        // combine bus reminders
        // we have for each bus now: a mod bus = r_bus
        // to combine two busses, we have:
        // I: a mod bus_1 = r_1
        // II: a mod bus_2 = r_2
        // we search a t of III, which solves II
        // III: a = r_1 + bus_1 * t (this one is always true for I)
        // inserting III into II
        // IV: (r1 + m1*t) mod m2 == r2
        // IV can be solved with fn calc_reminder(), see above:
        // t = calc_reminder(bus_1, bus_2, r1, r2)
        // than combine busses by multiplying them and
        // calc new_rem by inserting t in III.
        // do "new_rem mod new_m" to make sure, new_rem < new_m
        let init = bus_rem[0];
        let (_, rem) = bus_rem
            .into_iter()
            .skip(1)
            .fold(init, |(m1, r1), (m2, r2)| {
                let rem = calc_reminder(m1, m2, r1, r2);
                let new_m = m1 * m2;
                let new_rem = (r1 + m1 * rem).rem_euclid(new_m);
                (new_m, new_rem)
            });

        rem * self.busses[0].0
    }
    fn transform_to_reminder_form(&self) -> Vec<(i64, i64)> {
        let m1 = self.busses[0].0;
        self.busses
            .iter()
            .filter(|(_, d)| *d > 0)
            .map(|&(m2, r1)| (m2, calc_reminder(m1, m2, r1, 0)))
            .collect()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_13.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_13 part 1: {result_part1}");
    assert_eq!(result_part1, 410);

    let result_part2 = challenge.solution_part_2();
    println!("result day_13 part 2: {result_part2}");
    assert_eq!(result_part2, 600_691_418_730_595);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // with a LOOOOOOT of help of ChatGPT
    #[test]
    fn test_simple_example() {
        let ex = ChallengeInput {
            my_timestamp: 0,
            busses: vec![(3, 0), (7, 5), (5, 2)],
        };

        let rem = ex.transform_to_reminder_form();
        assert_eq!(rem[0].1, 3);
        assert_eq!(rem[1].1, 1);

        let sol = ex.solution_part_2();
        assert_eq!(sol, 31 * ex.busses[0].0);
    }

    #[test]
    fn test_example_day_13() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_13_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_13 part 1: {result_part1}");
        assert_eq!(result_part1, 295);

        let result_part2 = example.solution_part_2();
        println!("result day_13 part 2: {result_part2}");
        assert_eq!(result_part2, 1_068_781);

        Ok(())
    }
}
