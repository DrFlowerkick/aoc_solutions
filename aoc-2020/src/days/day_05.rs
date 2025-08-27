//!day_05.rs

use anyhow::Result;

struct BoardingPass {
    raw: String,
    row: u64,
    column: u64,
    seat_id: u64,
}

impl From<&str> for BoardingPass {
    fn from(value: &str) -> Self {
        BoardingPass {
            raw: value.to_string(),
            row: 0,
            column: 0,
            seat_id: 0,
        }
    }
}

impl BoardingPass {
    fn set_data_from_raw(&mut self) {
        self.row = 0;
        for c in self.raw.chars().take(7) {
            self.row <<= 1;
            match c {
                'F' => (),
                'B' => self.row += 1,
                _ => panic!("unknown char"),
            }
        }
        self.column = 0;
        for c in self.raw.chars().skip(7) {
            self.column <<= 1;
            match c {
                'L' => (),
                'R' => self.column += 1,
                _ => panic!("unknown char"),
            }
        }
        self.seat_id = self.row * 8 + self.column;
    }
    fn get_seat_bits_if_not_first_or_last_row(&self, last_row: u64) -> Option<u64> {
        (self.row > 0b0000000 && self.row < last_row).then_some((self.row << 3) + self.column)
    }
}

struct ChallengeInput {
    boarding_pass: Vec<BoardingPass>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            boarding_pass: value.lines().map(BoardingPass::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> u64 {
        self.boarding_pass.iter_mut().fold(0, |max, bp| {
            bp.set_data_from_raw();
            max.max(bp.seat_id)
        })
    }
    fn solution_part_2(&self) -> u64 {
        let last_row = self.boarding_pass.iter().map(|bp| bp.row).max().unwrap();
        let first_seat: u64 = 0b0000001000;
        let last_seat: u64 = 7 + ((last_row - 1) << 3);
        let sum_all_seats: u64 = (first_seat..=last_seat).sum();
        let sum_of_occupied_seats: u64 = self
            .boarding_pass
            .iter()
            .filter_map(|bp| bp.get_seat_bits_if_not_first_or_last_row(last_row))
            .sum();
        // my seat is the difference of sum of all seats and all occupied seats (not counting first and last row)
        let my_seat_bits = sum_all_seats - sum_of_occupied_seats;
        let my_column = my_seat_bits & 7;
        let my_row = my_seat_bits >> 3;
        my_row * 8 + my_column
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_05.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_05 part 1: {result_part1}");
    assert_eq!(result_part1, 813);

    let result_part2 = challenge.solution_part_2();
    println!("result day_05 part 2: {result_part2}");
    assert_eq!(result_part2, 612);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_05() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_05_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_05 part 1: {result_part1}");
        assert_eq!(result_part1, 820);

        let input = include_str!("../../../../aoc_input/aoc-2020/day_05.txt");
        let mut challenge = ChallengeInput::from(input);
        challenge.solution_part_1();

        // first row is not complete. Start in row 1
        let first_seat: u64 = 0b0000001000;
        let last_row = challenge
            .boarding_pass
            .iter()
            .map(|bp| bp.row)
            .max()
            .unwrap();
        // last row is not complete -> last full row = last_row -1
        let last_seat = ((last_row - 1) << 3) + 7;

        let all_seat_bits: Vec<u64> = challenge
            .boarding_pass
            .iter()
            .filter_map(|bp| bp.get_seat_bits_if_not_first_or_last_row(last_row))
            .collect();
        assert_eq!(*all_seat_bits.iter().min().unwrap(), first_seat);
        assert_eq!(*all_seat_bits.iter().max().unwrap(), last_seat);

        Ok(())
    }
}
