//!day_09.rs

use anyhow::Result;

// solution hint: use a recursive algorithmen
fn calc_next_in_sequence(sequence: &mut Vec<i64>) {
    if sequence.len() == 1 {
        panic!("day 09: sequence reduced to length 1");
    }
    let mut delta_sequence: Vec<i64> = Vec::with_capacity(sequence.len() - 1);
    let mut sequence_iter = sequence.iter();
    let mut last_value = *sequence_iter.next().unwrap();
    for value in sequence_iter {
        let result = *value - last_value;
        delta_sequence.push(result);
        last_value = *value;
    }
    if !delta_sequence.iter().any(|v| *v != 0) {
        // found final sequence; do final pushes because it fulfills the described algo
        delta_sequence.push(0);
        delta_sequence.push(0);
    } else {
        calc_next_in_sequence(&mut delta_sequence);
    }
    let last_delta = *delta_sequence.last().unwrap();
    let last = *sequence.last().unwrap();
    sequence.push(last + last_delta);
    let first_delta = delta_sequence[0];
    let first = sequence[0];
    sequence.insert(0, first - first_delta);
}

pub fn day_09() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_09.txt");

    let mut result_part1: i64 = 0;
    let mut result_part2: i64 = 0;
    for line in input.lines() {
        let mut sequence: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse::<i64>().expect("bad input"))
            .collect();
        calc_next_in_sequence(&mut sequence);
        result_part1 += *sequence.last().unwrap();
        result_part2 += sequence[0];
    }
    println!("result day 09 part 1: {}", result_part1);
    assert_eq!(result_part1, 2_038_472_161);
    println!("result day 09 part 2: {}", result_part2);
    assert_eq!(result_part2, 1_091);

    Ok(())
}
