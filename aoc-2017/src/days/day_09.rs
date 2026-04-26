//!day_09.rs

use anyhow::Result;

enum Stream {
    Group { value: u64, sub_stream: Vec<Stream> },
    Garbage { content: u64 },
}

impl Stream {
    fn new() -> Self {
        Stream::Group {
            value: 1,
            sub_stream: vec![],
        }
    }
    fn new_group(value: u64) -> Self {
        Stream::Group {
            value,
            sub_stream: vec![],
        }
    }
    fn new_garbage() -> Self {
        Stream::Garbage { content: 0 }
    }
    fn parse(&mut self, stream_iter: &mut impl Iterator<Item = char>) {
        match self {
            Self::Group { value, sub_stream } => {
                while let Some(c) = stream_iter.next() {
                    match c {
                        '}' => return,
                        '{' => {
                            let mut new_group = Stream::new_group(*value + 1);
                            new_group.parse(stream_iter);
                            sub_stream.push(new_group);
                        }
                        '<' => {
                            let mut new_garbage = Stream::new_garbage();
                            new_garbage.parse(stream_iter);
                            sub_stream.push(new_garbage);
                        }
                        _ => (),
                    }
                }
            }
            Self::Garbage { content } => {
                let mut ignore = false;
                for c in stream_iter.by_ref() {
                    if ignore {
                        ignore = false;
                        continue;
                    }
                    match c {
                        '>' => return,
                        '!' => {
                            ignore = true;
                        }
                        _ => *content += 1,
                    }
                }
            }
        }
    }
    fn total_score(&self) -> u64 {
        match self {
            Stream::Group { value, sub_stream } => {
                *value + sub_stream.iter().map(|s| s.total_score()).sum::<u64>()
            }
            Stream::Garbage { .. } => 0,
        }
    }
    fn total_garbage(&self) -> u64 {
        match self {
            Stream::Group { sub_stream, .. } => sub_stream.iter().map(|s| s.total_garbage()).sum(),
            Stream::Garbage { content } => *content,
        }
    }
}

struct ChallengeInput<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        ChallengeInput { input: value }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1_and_2(&self) -> (u64, u64) {
        let mut stream = Stream::new();
        // skip first char, because we already created initial group
        stream.parse(&mut self.input.chars().skip(1));
        (stream.total_score(), stream.total_garbage())
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2017/day_09.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_09 part 1: {result_part1}");
    assert_eq!(result_part1, 10_616);

    println!("result day_09 part 2: {result_part2}");
    assert_eq!(result_part2, 5_101);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_09() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2017/day_09_example.txt");
        let example = ChallengeInput::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_09 part 1: {result_part1}");
        assert_eq!(result_part1, 3);

        println!("result day_09 part 2: {result_part2}");
        assert_eq!(result_part2, 17);

        Ok(())
    }
}
