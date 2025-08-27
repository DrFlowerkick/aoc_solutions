//!day_10.rs

use anyhow::Result as AnyResult;
use std::str::Chars;

#[derive(PartialEq, Debug)]
enum ChunkError {
    IllegalRB,
    IllegalSB,
    IllegalBB,
    IllegalGB,
}

impl ChunkError {
    fn syntax_error_score(&self) -> u64 {
        match self {
            ChunkError::IllegalRB => 3,
            ChunkError::IllegalSB => 57,
            ChunkError::IllegalBB => 1197,
            ChunkError::IllegalGB => 25137,
        }
    }
}

struct ChallengeInput {
    chunks: String,
    completion_strings: Vec<String>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            chunks: value.into(),
            completion_strings: Vec::new(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> u64 {
        self.chunks
            .lines()
            .filter_map(|chunk| {
                let mut chunk_iter = chunk.chars();
                let mut completion_string = String::new();
                match ChallengeInput::check_chunk(&mut chunk_iter, None, &mut completion_string) {
                    Ok(()) => {
                        if !completion_string.is_empty() {
                            self.completion_strings.push(completion_string);
                        }
                        None
                    }
                    Err(err) => Some(err.syntax_error_score()),
                }
            })
            .sum()
    }
    fn check_chunk(
        chunk_iter: &mut Chars,
        expected_brace: Option<char>,
        completion_string: &mut String,
    ) -> Result<(), ChunkError> {
        let Some(current_brace) = chunk_iter.next() else {
            if let Some(eb) = expected_brace {
                completion_string.push(eb)
            }
            return Ok(());
        };
        match expected_brace {
            Some(eb) => {
                if eb == current_brace {
                    return Ok(());
                }
                match current_brace {
                    ')' => return Err(ChunkError::IllegalRB),
                    ']' => return Err(ChunkError::IllegalSB),
                    '}' => return Err(ChunkError::IllegalBB),
                    '>' => return Err(ChunkError::IllegalGB),
                    _ => (),
                }
            }
            None => match current_brace {
                ')' | ']' | '}' | '>' => panic!("unexpected closing bracket"),
                _ => (),
            },
        }
        let eb = match current_brace {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("unknown char"),
        };
        ChallengeInput::check_chunk(chunk_iter, Some(eb), completion_string)?;
        ChallengeInput::check_chunk(chunk_iter, expected_brace, completion_string)
    }
    fn solution_part_2(&self) -> u64 {
        let mut completion_scores: Vec<u64> = self
            .completion_strings
            .iter()
            .map(|cs| {
                let mut completion_score = 0;
                for bracket in cs.chars() {
                    completion_score *= 5;
                    completion_score += match bracket {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    }
                }
                completion_score
            })
            .collect();
        // check odd number of completion scores
        assert!(completion_scores.len() & 1 == 1);
        completion_scores.sort();
        completion_scores[completion_scores.len() / 2]
    }
}

pub fn solution() -> AnyResult<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_10.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_10 part 1: {result_part1}");
    assert_eq!(result_part1, 392_421);

    let result_part2 = challenge.solution_part_2();
    println!("result day_10 part 2: {result_part2}");
    assert_eq!(result_part2, 2_769_449_099);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_chunk() {
        let mut completion_string = String::new();
        let sample_1 = "(]";
        let mut sample_iter = sample_1.chars();
        assert_eq!(
            ChallengeInput::check_chunk(&mut sample_iter, None, &mut completion_string),
            Err(ChunkError::IllegalSB)
        );

        let sample_2 = "{()()()>";
        let mut sample_iter = sample_2.chars();
        assert_eq!(
            ChallengeInput::check_chunk(&mut sample_iter, None, &mut completion_string),
            Err(ChunkError::IllegalGB)
        );

        let sample_3 = "(((()))}";
        let mut sample_iter = sample_3.chars();
        assert_eq!(
            ChallengeInput::check_chunk(&mut sample_iter, None, &mut completion_string),
            Err(ChunkError::IllegalBB)
        );

        let sample_4 = "<([]){()}[{}])";
        let mut sample_iter = sample_4.chars();
        assert_eq!(
            ChallengeInput::check_chunk(&mut sample_iter, None, &mut completion_string),
            Err(ChunkError::IllegalRB)
        );

        let sample_5 = "[<>({}){}[([])<>]]";
        let mut sample_iter = sample_5.chars();
        assert_eq!(
            ChallengeInput::check_chunk(&mut sample_iter, None, &mut completion_string),
            Ok(())
        );

        assert!(completion_string.is_empty());

        let sample_6 = "[({(<(())[]>[[{[]{<()<>>";
        let mut sample_iter = sample_6.chars();
        ChallengeInput::check_chunk(&mut sample_iter, None, &mut completion_string).unwrap();
        assert_eq!(completion_string, "}}]])})]");
    }

    #[test]
    fn test_example_part() -> AnyResult<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_10_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_10 part 1: {result_part1}");
        assert_eq!(result_part1, 26_397);

        let result_part2 = example.solution_part_2();
        println!("result day_10 part 2: {result_part2}");
        assert_eq!(result_part2, 288_957);

        Ok(())
    }
}
