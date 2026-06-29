//!lib.rs

pub mod days;

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

    Ok(())
}
