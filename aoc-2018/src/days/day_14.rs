//!day_14.rs

use anyhow::Result;
use std::{cell::RefCell, fmt::Write, rc::Rc};

struct Recipe {
    id: u64,
    score: u32,
    previous: RefCell<Option<Rc<Recipe>>>,
    next: RefCell<Option<Rc<Recipe>>>,
}

impl Recipe {
    fn new(id: u64, score: u32) -> Rc<Recipe> {
        let new = Rc::new(Recipe {
            id,
            score,
            previous: RefCell::new(None),
            next: RefCell::new(None),
        });
        new.relink_next(new.clone());
        new.relink_previous(new.clone());
        new
    }
    fn next(&self) -> Rc<Recipe> {
        self.next.borrow().as_ref().unwrap().clone()
    }
    fn previous(&self) -> Rc<Recipe> {
        self.previous.borrow().as_ref().unwrap().clone()
    }
    fn relink_next(&self, next: Rc<Recipe>) {
        *self.next.borrow_mut() = Some(next.clone());
    }
    fn relink_previous(&self, previous: Rc<Recipe>) {
        *self.previous.borrow_mut() = Some(previous.clone());
    }
    fn append_to_last(&self, new_last: Rc<Recipe>) {
        let first = self.next();
        let last = first.previous();
        last.relink_next(new_last.clone());
        new_last.relink_previous(last);
        new_last.relink_next(first.clone());
        first.relink_previous(new_last);
    }
    fn move_n_next(&self, n: u32) -> Option<Rc<Recipe>> {
        if n == 0 {
            return None;
        }
        let next = self.next();
        if n - 1 == 0 {
            return Some(next);
        }
        next.move_n_next(n - 1)
    }
    fn append_next_ten_recipe_scores(&self, scores: &mut String) {
        if scores.len() < 10 {
            let next = self.next();
            write!(scores, "{}", next.score).unwrap();
            next.append_next_ten_recipe_scores(scores);
        }
    }
    fn compare_pattern(&self, pattern_iter: &mut impl Iterator<Item = char>) -> Option<u64> {
        if let Some(c) = pattern_iter.next() {
            let digit = c.to_digit(10).unwrap();
            if self.score != digit {
                return None;
            }
            let previous = self.previous();
            if previous.id > self.id {
                return None;
            }
            return previous.compare_pattern(pattern_iter);
        }
        Some(self.id + 1)
    }
}
struct ChallengeInput {
    num_recipes: u64,
    pattern: String,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            num_recipes: value.parse().unwrap(),
            pattern: value.to_string(),
        }
    }
}

// NOTE: I could combine solution_part_1 and solution_part_2, but the example inputs
// for part 1 and part 2 are not compatible for a combined solution.
impl ChallengeInput {
    fn solution_part_1(&self) -> String {
        let mut first = Recipe::new(0, 3);
        let mut second = Recipe::new(1, 7);
        first.append_to_last(second.clone());
        let mut last = second.clone();
        let mut before_puzzle_score = second.clone();
        let mut recipe_count: u64 = 2;
        loop {
            // add new recipe(s)
            let score_sum = first.score + second.score;
            if score_sum > 9 {
                let ten_digit = score_sum / 10;
                let new = Recipe::new(recipe_count, ten_digit);
                last.append_to_last(new.clone());
                recipe_count += 1;
                if recipe_count == self.num_recipes {
                    before_puzzle_score = new.clone();
                }
                last = new;
            }
            let one_digit = score_sum % 10;
            let new = Recipe::new(recipe_count, one_digit);
            last.append_to_last(new.clone());
            recipe_count += 1;
            if recipe_count == self.num_recipes {
                before_puzzle_score = new.clone();
            }
            last = new;
            // check for puzzle solution
            if recipe_count >= self.num_recipes + 10 {
                let mut scores = String::new();
                before_puzzle_score.append_next_ten_recipe_scores(&mut scores);
                return scores;
            }
            // move first and second
            first = first.move_n_next(first.score + 1).unwrap();
            second = second.move_n_next(second.score + 1).unwrap();
        }
    }
    fn solution_part_2(&self) -> u64 {
        let mut first = Recipe::new(0, 3);
        let mut second = Recipe::new(1, 7);
        first.append_to_last(second.clone());
        let mut last = second.clone();
        let mut recipe_count: u64 = 2;
        loop {
            // add new recipe(s)
            let score_sum = first.score + second.score;
            if score_sum > 9 {
                let ten_digit = score_sum / 10;
                let new = Recipe::new(recipe_count, ten_digit);
                last.append_to_last(new.clone());
                recipe_count += 1;
                last = new;
                if let Some(count) = last.compare_pattern(&mut self.pattern.chars().rev()) {
                    return count;
                }
            }
            let one_digit = score_sum % 10;
            let new = Recipe::new(recipe_count, one_digit);
            last.append_to_last(new.clone());
            recipe_count += 1;
            last = new;
            if let Some(count) = last.compare_pattern(&mut self.pattern.chars().rev()) {
                return count;
            }
            // move first and second
            first = first.move_n_next(first.score + 1).unwrap();
            second = second.move_n_next(second.score + 1).unwrap();
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_14.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_14 part 1: {result_part1}");
    assert_eq!(result_part1, "1132413111");

    let result_part2 = challenge.solution_part_2();
    println!("result day_14 part 2: {result_part2}");
    assert_eq!(result_part2, 20_340_232);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_14() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_14_example.txt");

        let example_data = [
            ("5158916779", "51589"),
            ("0124515891", "01245"),
            ("9251071085", "92510"),
            ("5941429882", "59414"),
        ];

        for (line, (solution_part_1, input_part_2)) in input.lines().zip(example_data) {
            let example = ChallengeInput::from(line);

            let result_part1 = example.solution_part_1();
            println!("result day_14 part 1: {result_part1}");
            assert_eq!(result_part1, solution_part_1);

            let example = ChallengeInput::from(input_part_2);
            let solution_part_2: u64 = line.parse().unwrap();
            let result_part2 = example.solution_part_2();
            println!("result day_14 part 2: {result_part2}");
            assert_eq!(result_part2, solution_part_2);
        }

        Ok(())
    }
}
