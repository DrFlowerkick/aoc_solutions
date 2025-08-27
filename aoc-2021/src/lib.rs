//!lib.rs

pub mod days;

use anyhow::Result;

pub fn run() -> Result<()> {
    days::day_01::day_01()?;
    days::day_02::day_02()?;
    days::day_03::day_03()?;
    days::day_04::day_04()?;
    days::day_05::day_05()?;
    days::day_06::day_06()?;
    days::day_07::day_07()?;
    days::day_08::day_08()?;
    days::day_09::solution()?;
    days::day_10::solution()?;
    days::day_11::solution()?;
    days::day_12::solution()?;
    days::day_13::solution()?;
    days::day_14::solution()?;
    days::day_15::solution()?;
    days::day_16::solution()?;
    days::day_17::solution()?;
    days::day_18::solution()?;
    days::day_19::solution()?;
    days::day_20::solution()?;
    days::day_21::solution()?;
    days::day_22::solution()?;
    days::day_23::solution()?;
    // day 24 solved in libre office calc, which contains the puzzle input,
    // which is why it is not included in this repo.
    days::day_25::solution()?;

    Ok(())
}
