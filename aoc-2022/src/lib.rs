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
    days::day_09::day_09()?;
    days::day_10::day_10()?;
    days::day_11::day_11()?;
    days::day_12::day_12()?;
    days::day_13::day_13()?;
    days::day_14::day_14()?;
    days::day_15::day_15()?;
    days::day_16::day_16()?;
    days::day_17::day_17()?;
    days::day_18::day_18()?;
    days::day_19::day_19()?;
    days::day_20::day_20()?;
    days::day_21::day_21()?;
    days::day_22::day_22()?;
    days::day_23::day_23()?;
    days::day_24::day_24()?;
    days::day_25::day_25()?;

    Ok(())
}
