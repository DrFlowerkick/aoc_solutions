//!day_04.rs

use anyhow::Result;
use my_lib::my_map_two_dim::MyMap2D;

struct BingoGame {
    numbers: Vec<u64>,
    boards: Vec<MyMap2D<u64, 5, 5>>,
}

impl From<&str> for BingoGame {
    fn from(value: &str) -> Self {
        let (numbers, boards) = value.split_once("\n\n").unwrap();
        let numbers: Vec<u64> = numbers.split(',').filter_map(|n| n.parse().ok()).collect();
        let boards: Vec<MyMap2D<u64, 5, 5>> = boards
            .split("\n\n")
            .map(|b| {
                let mut map = MyMap2D::default();
                for (y, line) in b.lines().enumerate() {
                    for (x, c) in line.split_ascii_whitespace().enumerate() {
                        let num: u64 = c.parse().unwrap();
                        map.set((x, y).into(), num);
                    }
                }
                map
            })
            .collect();

        BingoGame { numbers, boards }
    }
}

impl BingoGame {
    fn find_first_winner(&self) -> u64 {
        for len in 5..=self.numbers.len() {
            let draws = &self.numbers[..len];
            for (board_index, board) in self.boards.iter().enumerate() {
                for index in 0..5 {
                    if board.iter_row(index).all(|(_, num)| draws.contains(num)) {
                        return self.calc_board_score(board_index, draws);
                    }
                    if board.iter_column(index).all(|(_, num)| draws.contains(num)) {
                        return self.calc_board_score(board_index, draws);
                    }
                }
            }
        }
        panic!("Could not find a winner");
    }

    fn find_last_winner(&self) -> u64 {
        let mut last_win_draws: Option<Vec<u64>> = None;
        let mut winners: Vec<usize> = Vec::with_capacity(self.boards.len());
        for len in 5..=self.numbers.len() {
            let draws = &self.numbers[..len];
            for (board_index, board) in self.boards.iter().enumerate() {
                if winners.contains(&board_index) {
                    continue;
                }
                for index in 0..5 {
                    if board.iter_row(index).all(|(_, num)| draws.contains(num)) {
                        winners.push(board_index);
                        last_win_draws = Some(draws.into());
                        break;
                    }
                    if board.iter_column(index).all(|(_, num)| draws.contains(num)) {
                        winners.push(board_index);
                        last_win_draws = Some(draws.into());
                        break;
                    }
                }
            }
        }
        let last_win_draws = last_win_draws.expect("Could not find a winner");
        let board_index = winners.last().unwrap();
        self.calc_board_score(*board_index, &last_win_draws)
    }

    fn calc_board_score(&self, board_index: usize, draws: &[u64]) -> u64 {
        let sum_unmarked_num: u64 = self.boards[board_index]
            .iter()
            .filter(|(_, n)| !draws.contains(n))
            .map(|(_, n)| n)
            .sum();
        sum_unmarked_num * draws.last().unwrap()
    }
}

pub fn day_04() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_04.txt");
    let bingo = BingoGame::from(input);

    let result_part1 = bingo.find_first_winner();
    println!("result day_04 part 1: {result_part1}");
    assert_eq!(result_part1, 38_594);

    let result_part2 = bingo.find_last_winner();
    println!("result day_04 part 2: {result_part2}");
    assert_eq!(result_part2, 21_184);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_04_example.txt");
        let bingo = BingoGame::from(input);

        let result_part1 = bingo.find_first_winner();
        println!("result day_04 part 1: {result_part1}");
        assert_eq!(result_part1, 4_512);

        let result_part2 = bingo.find_last_winner();
        println!("result day_04 part 2: {result_part2}");
        assert_eq!(result_part2, 1_924);

        Ok(())
    }
}
