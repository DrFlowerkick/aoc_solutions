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

    Ok(())
}
