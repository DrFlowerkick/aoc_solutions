//!day_06.rs

use anyhow::{Result, anyhow};

struct RaceParam {
    time: f64,
    distance: f64,
}

impl RaceParam {
    fn new(time: f64, distance: f64) -> Self {
        Self { time, distance }
    }
    fn calc_times(&self) -> (u64, u64) {
        let time_1 = ((self.time / 2.0) - ((self.time / 2.0).powf(2.0) - self.distance).sqrt())
            .ceil() as u64;
        let time_2 = ((self.time / 2.0) + ((self.time / 2.0).powf(2.0) - self.distance).sqrt())
            .floor() as u64;
        (time_1, time_2)
    }
}

pub fn day_06() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_06.txt");
    let mut input_iter = input.lines();
    let times = input_iter
        .next()
        .ok_or(anyhow!("bad input"))?
        .split_once(':')
        .ok_or(anyhow!("bad input"))?
        .1
        .trim();
    let distances = input_iter
        .next()
        .ok_or(anyhow!("bad input"))?
        .split_once(':')
        .ok_or(anyhow!("bad input"))?
        .1
        .trim();
    let mut races: Vec<RaceParam> = Vec::with_capacity(4);
    for (time, distance) in times
        .split_ascii_whitespace()
        .map(|t| t.parse::<f64>().expect("bad input"))
        .zip(
            distances
                .split_ascii_whitespace()
                .map(|d| d.parse::<f64>().expect("bad input")),
        )
    {
        races.push(RaceParam::new(time, distance));
    }

    let mut result_part1: u64 = 1;
    for race in races.iter() {
        let (time_1, time_2) = race.calc_times();
        result_part1 *= time_2 - time_1 + 1;
    }
    println!("result day 06 part 1: {}", result_part1);
    assert_eq!(result_part1, 345_015);

    // part 2
    let mut input_iter = input.lines();
    let time = String::from_iter(
        input_iter
            .next()
            .ok_or(anyhow!("bad input"))?
            .split_once(':')
            .ok_or(anyhow!("bad input"))?
            .1
            .trim()
            .split_ascii_whitespace(),
    )
    .parse::<f64>()?;
    let distance = String::from_iter(
        input_iter
            .next()
            .ok_or(anyhow!("bad input"))?
            .split_once(':')
            .ok_or(anyhow!("bad input"))?
            .1
            .trim()
            .split_ascii_whitespace(),
    )
    .parse::<f64>()?;
    let race = RaceParam::new(time, distance);
    let (time_1, time_2) = race.calc_times();
    let result_part2 = time_2 - time_1 + 1;
    println!("result day 06 part 2: {}", result_part2);
    assert_eq!(result_part2, 42_588_603);

    Ok(())
}
