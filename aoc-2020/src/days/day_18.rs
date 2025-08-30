//!day_18.rs

use anyhow::Result;

struct ChallengeInput {
    expression_list: String,
    open: char,
    close: char,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            expression_list: value.to_string(),
            open: '(',
            close: ')',
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.expression_list
            .lines()
            .map(|exp| {
                let mut chars = exp.chars().filter(|c| !c.is_whitespace());
                self.calc_expression(&mut chars)
            })
            .sum()
    }
    fn solution_part_2(&self) -> u64 {
        self.expression_list
            .lines()
            .map(|exp| {
                let mut chars = exp.chars().filter(|c| !c.is_whitespace());
                self.calc_expression_adv(&mut chars)
            })
            .sum()
    }
    fn calc_expression(&self, chars: &mut impl Iterator<Item = char>) -> u64 {
        let mut res = 0;
        let mut first_num = true;
        let mut last_operator = false;
        let mut expect_num = true;
        while let Some(c) = chars.next() {
            match c {
                open if self.open == open => {
                    if first_num {
                        res = self.calc_expression(chars);
                        first_num = false;
                    } else if last_operator {
                        res *= self.calc_expression(chars);
                    } else {
                        res += self.calc_expression(chars);
                    }
                }
                close if self.close == close => return res,
                val if val.is_ascii_digit() => {
                    if !expect_num {
                        panic!("detected num with more than one digit");
                    }
                    let num = c.to_digit(10).unwrap() as u64;
                    if first_num {
                        res = num;
                        first_num = false
                    } else if last_operator {
                        res *= num;
                    } else {
                        res += num;
                    }
                    expect_num = false;
                }
                '+' => {
                    last_operator = false;
                    expect_num = true;
                }
                '*' => {
                    last_operator = true;
                    expect_num = true;
                }
                _ => panic!("unexpected char."),
            }
        }
        res
    }
    fn calc_expression_adv(&self, chars: &mut impl Iterator<Item = char>) -> u64 {
        let mut res = 0;
        let mut first_num = true;
        let mut expect_num = true;
        while let Some(c) = chars.next() {
            match c {
                open if self.open == open => {
                    if first_num {
                        res = self.calc_expression_adv(chars);
                        first_num = false;
                    } else {
                        res += self.calc_expression_adv(chars);
                    }
                }
                close if self.close == close => return res,
                val if val.is_ascii_digit() => {
                    if !expect_num {
                        panic!("detected num with more than one digit");
                    }
                    let num = c.to_digit(10).unwrap() as u64;
                    if first_num {
                        res = num;
                        first_num = false;
                    } else {
                        res += num;
                    }
                    expect_num = false;
                }
                '+' => {
                    expect_num = true;
                }
                '*' => {
                    res *= self.calc_expression_adv(chars);
                    return res;
                }
                _ => panic!("unexpected char."),
            }
        }
        res
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_18.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_18 part 1: {result_part1}");
    assert_eq!(result_part1, 9_535_936_849_815);

    let result_part2 = challenge.solution_part_2();
    println!("result day_18 part 2: {result_part2}");
    assert_eq!(result_part2, 472_171_581_333_710);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_18() -> Result<()> {
        let input_lines = include_str!("../../../../aoc_input/aoc-2020/day_18_example.txt");
        let solutions = [
            (71, 231),
            (51, 51),
            (26, 46),
            (437, 1445),
            (12_240, 669_060),
            (13_632, 23_340),
        ];
        for (input, (solution_1, solution_2)) in input_lines.lines().zip(solutions) {
            let example = ChallengeInput::from(input);

            let result_part1 = example.solution_part_1();
            println!("result day_18 part 1: {result_part1}");
            assert_eq!(result_part1, solution_1);

            let result_part2 = example.solution_part_2();
            println!("result day_18 part 2: {result_part2}");
            assert_eq!(result_part2, solution_2);
        }

        Ok(())
    }
}
