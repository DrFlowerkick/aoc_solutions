//!day_21.rs

use super::day_05::IntCodeComputer;
use anyhow::Result;

struct ChallengeInput {
    code: IntCodeComputer,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            code: IntCodeComputer::from(value),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> Result<i64> {
        // jump if there is a hole at A, B or C AND D is no hole
        let spring_script = "NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\nWALK\n";
        self.jump_droid(spring_script)
    }
    fn solution_part_2(&self) -> Result<i64> {
        // Jump if B or C is hole AND D is no hole AND (E is no hole OR H is no hole); jump if A is hole.
        // Use "NOT J T\n" to reset temporary register to false, if possible jump situation is detected.
        // This reset is required for "OR E T\nOR H T\n" to work (if T would be true, the values of E or H would be irrelevant).
        // If J is false, this reset would set T to true, but since we do "AND T J\n", J would still be false.
        let spring_script = "NOT B J\nNOT C T\nOR T J\nAND D J\nNOT J T\nOR E T\nOR H T\nAND T J\nNOT A T\nOR T J\nRUN\n";
        self.jump_droid(spring_script)
    }
    fn jump_droid(&self, spring_script: &str) -> Result<i64> {
        let mut jump_droid = self.code.clone();
        let mut input: Vec<i64> = Vec::new();
        while let Some(ascii) = jump_droid
            .run_int_code(&input)
            .map_err(|err| anyhow::anyhow!("{err}"))?
        {
            if ascii > 255 {
                return Ok(ascii);
            }
            let ch = (ascii as u8) as char;
            if ch == ':' {
                input = spring_script.chars().map(|c| c as i64).collect();
            }
            // uncomment for debug
            //print!("{ch}");
        }
        Ok(0)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_21.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1()?;
    println!("result day_21 part 1: {result_part1}");
    assert_eq!(result_part1, 19_348_840);

    let result_part2 = challenge.solution_part_2()?;
    println!("result day_21 part 2: {result_part2}");
    assert_eq!(result_part2, 1_141_857_182);

    Ok(())
}

#[cfg(test)]
mod tests {
    /*  int code challenge does not provide example
    use super::*;

    #[test]
    fn test_example_day_21() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_21_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_21 part 1: {result_part1}");
        //assert_eq!(result_part1, XXX);

        let result_part2 = example.solution_part_2();
        println!("result day_21 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }*/
}
