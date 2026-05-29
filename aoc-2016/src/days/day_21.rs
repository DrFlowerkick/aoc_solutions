//!day_21.rs

use anyhow::Result;

#[derive(Clone, Copy)]
enum Action {
    SwapPos(usize, usize),
    SwapChar(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateByPosOfChar(char),
    ReverseAllPosBetween(usize, usize),
    MovePosToNewPos(usize, usize),
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        if let Some(rem_swap_pos) = value.strip_prefix("swap position ") {
            let (x, y) = rem_swap_pos.split_once(" with position ").unwrap();
            Action::SwapPos(x.parse().unwrap(), y.parse().unwrap())
        } else if let Some(rem_swap_char) = value.strip_prefix("swap letter ") {
            let (x, y) = rem_swap_char.split_once(" with letter ").unwrap();
            Action::SwapChar(x.parse().unwrap(), y.parse().unwrap())
        } else if let Some(rem_rot_left) = value.strip_prefix("rotate left ") {
            let x = rem_rot_left.split_whitespace().next().unwrap();
            Action::RotateLeft(x.parse().unwrap())
        } else if let Some(rem_rot_right) = value.strip_prefix("rotate right ") {
            let x = rem_rot_right.split_whitespace().next().unwrap();
            Action::RotateRight(x.parse().unwrap())
        } else if let Some(rem_rot_by_pos_of_char) =
            value.strip_prefix("rotate based on position of letter ")
        {
            Action::RotateByPosOfChar(rem_rot_by_pos_of_char.parse().unwrap())
        } else if let Some(rem_reverse_all_pos_between) = value.strip_prefix("reverse positions ") {
            let (x, y) = rem_reverse_all_pos_between.split_once(" through ").unwrap();
            Action::ReverseAllPosBetween(x.parse().unwrap(), y.parse().unwrap())
        } else if let Some(rem_move_pos_to_new_pos) = value.strip_prefix("move position ") {
            let (x, y) = rem_move_pos_to_new_pos.split_once(" to position ").unwrap();
            Action::MovePosToNewPos(x.parse().unwrap(), y.parse().unwrap())
        } else {
            panic!("unknown action")
        }
    }
}

impl Action {
    fn apply(&self, slice: &mut [char]) {
        match *self {
            Action::SwapPos(x, y) => {
                slice.swap(x, y);
            }
            Action::SwapChar(x, y) => {
                let x = slice.iter().position(|c| *c == x).unwrap();
                let y = slice.iter().position(|c| *c == y).unwrap();
                slice.swap(x, y);
            }
            Action::RotateLeft(rl) => {
                slice.rotate_left(rl % slice.len());
            }
            Action::RotateRight(rr) => {
                slice.rotate_right(rr % slice.len());
            }
            Action::RotateByPosOfChar(cop) => {
                let mut pos = slice.iter().position(|c| *c == cop).unwrap();
                if pos >= 4 {
                    pos += 1;
                }
                slice.rotate_right((pos + 1) % slice.len());
            }
            Action::ReverseAllPosBetween(x, y) => {
                let min = x.min(y);
                let max = x.max(y);
                slice[min..=max].reverse();
            }
            Action::MovePosToNewPos(x, y) => {
                if x == y {
                    return;
                }
                let min = x.min(y);
                let max = x.max(y);
                if x < y {
                    slice[min..=max].rotate_left(1);
                } else {
                    slice[min..=max].rotate_right(1);
                }
            }
        }
    }
    fn reverse(&self, slice: &mut [char]) {
        match *self {
            Action::SwapPos(x, y) => {
                slice.swap(y, x);
            }
            Action::SwapChar(x, y) => {
                let x = slice.iter().position(|c| *c == x).unwrap();
                let y = slice.iter().position(|c| *c == y).unwrap();
                slice.swap(y, x);
            }
            Action::RotateLeft(rl) => {
                slice.rotate_right(rl % slice.len());
            }
            Action::RotateRight(rr) => {
                slice.rotate_left(rr % slice.len());
            }
            Action::RotateByPosOfChar(cop) => {
                let pos = slice.iter().position(|c| *c == cop).unwrap();
                let rot = match pos {
                    1 => 1,
                    3 => 2,
                    5 => 3,
                    7 => 4,
                    2 => 6,
                    4 => 7,
                    6 => 0,
                    0 => 1,
                    _ => panic!("unexpected pos"),
                };
                slice.rotate_left(rot);
            }
            Action::ReverseAllPosBetween(x, y) => {
                let min = x.min(y);
                let max = x.max(y);
                slice[min..=max].reverse();
            }
            Action::MovePosToNewPos(x, y) => {
                if x == y {
                    return;
                }
                let min = x.min(y);
                let max = x.max(y);
                if x < y {
                    slice[min..=max].rotate_right(1);
                } else {
                    slice[min..=max].rotate_left(1);
                }
            }
        }
    }
}

struct ChallengeInput {
    actions: Vec<Action>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            actions: value.lines().map(Action::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self, password: &str) -> String {
        let mut pwd: Vec<char> = password.chars().collect();
        for action in self.actions.iter() {
            action.apply(&mut pwd[..]);
        }
        pwd.into_iter().collect()
    }
    fn solution_part_2(&self, scrambled: &str) -> String {
        let mut pwd: Vec<char> = scrambled.chars().collect();
        for action in self.actions.iter().rev() {
            action.reverse(&mut pwd[..]);
        }
        pwd.into_iter().collect()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2016/day_21.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1("abcdefgh");
    println!("result day_21 part 1: {result_part1}");
    assert_eq!(result_part1, "cbeghdaf");

    let result_part2 = challenge.solution_part_2("fbgdceah");
    println!("result day_21 part 2: {result_part2}");
    assert_eq!(result_part2, "bacdefgh");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_21() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2016/day_21_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1("abcde");
        println!("result day_21 part 1: {result_part1}");
        assert_eq!(result_part1, "decab");

        let result_part2 = example.solution_part_2("decab");
        println!("result day_21 part 2: {result_part2}");
        assert_eq!(result_part2, "abcde");

        Ok(())
    }
}
