//!lib.rs

pub mod days;

pub mod day_25_tui;

use anyhow::Result;

pub fn run() -> Result<()> {
    days::day_01::solution()?;
    days::day_02::solution()?;
    days::day_03::solution()?;
    days::day_04::solution()?;
    days::day_05::solution()?;
    days::day_06::solution()?;
    days::day_07::solution()?;
    days::day_08::solution()?;
    days::day_09::solution()?;
    days::day_10::solution()?;
    days::day_11::solution()?;
    days::day_12::solution()?;
    days::day_13::solution()?;
    days::day_14::solution()?;
    days::day_15::solution()?;
    days::day_16::solution()?;
    days::day_17::solution()?;
    #[cfg(any(feature = "long-run-time", test))]
    days::day_18::solution()?;
    #[cfg(not(feature = "long-run-time"))]
    {
        println!("day 18 part 1 and 2 skipped because of long run time")
    }
    days::day_19::solution()?;
    days::day_20::solution()?;
    days::day_21::solution()?;
    days::day_22::solution()?;
    days::day_23::solution()?;
    days::day_24::solution()?;
    println!(
        "Use day_25_tui to solve day 25. Hint: the crawler in the tui solves it automatically for you. Happy Christmas🎄🎁🎁🎁"
    );

    Ok(())
}
