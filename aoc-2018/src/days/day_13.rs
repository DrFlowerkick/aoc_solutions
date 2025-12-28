//!day_13.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy)]
enum IntersectionAction {
    Left,
    Straight,
    Right,
}

impl IntersectionAction {
    fn next(&self) -> Self {
        match self {
            Self::Left => Self::Straight,
            Self::Straight => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cart {
    direction: Compass,
    next_intersection_action: IntersectionAction,
}

impl From<char> for Cart {
    fn from(value: char) -> Self {
        let direction = match value {
            '^' => Compass::N,
            '>' => Compass::E,
            'v' => Compass::S,
            '<' => Compass::W,
            _ => unreachable!(),
        };
        Cart {
            direction,
            next_intersection_action: IntersectionAction::Left,
        }
    }
}

impl Cart {
    fn tick(mut self, pos: &Point, cart_mine: &HashMap<Point, char>) -> (Point, Cart) {
        match cart_mine.get(pos).unwrap() {
            '|' | '-' => (),
            '/' => {
                let new_direction = match self.direction {
                    Compass::N => Compass::E,
                    Compass::E => Compass::N,
                    Compass::S => Compass::W,
                    Compass::W => Compass::S,
                    _ => unreachable!(),
                };
                self.direction = new_direction;
            }
            '\\' => {
                let new_direction = match self.direction {
                    Compass::N => Compass::W,
                    Compass::W => Compass::N,
                    Compass::S => Compass::E,
                    Compass::E => Compass::S,
                    _ => unreachable!(),
                };
                self.direction = new_direction;
            }
            '+' => {
                let new_direction = match self.next_intersection_action {
                    IntersectionAction::Straight => self.direction,
                    IntersectionAction::Left => {
                        self.direction.counterclockwise().counterclockwise()
                    }
                    IntersectionAction::Right => self.direction.clockwise().clockwise(),
                };
                self.direction = new_direction;
                self.next_intersection_action = self.next_intersection_action.next();
            }
            _ => unreachable!(),
        }
        (pos.add(self.direction), self)
    }
}

struct ChallengeInput {
    cart_mine: HashMap<Point, char>,
    carts: BTreeMap<Point, Cart>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut cart_mine = HashMap::new();
        let mut carts = BTreeMap::new();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Point::new(x as i64, y as i64);
                match c {
                    '|' | '-' | '+' | '/' | '\\' => {
                        cart_mine.insert(pos, c);
                    }
                    '^' | 'v' => {
                        cart_mine.insert(pos, '|');
                        let cart = Cart::from(c);
                        carts.insert(pos, cart);
                    }
                    '<' | '>' => {
                        cart_mine.insert(pos, '-');
                        let cart = Cart::from(c);
                        carts.insert(pos, cart);
                    }
                    _ => (),
                }
            }
        }
        ChallengeInput { cart_mine, carts }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> String {
        loop {
            let crashes = self.tick();
            if let Some(crash_pos) = crashes.first() {
                return format!("{},{}", crash_pos.x, crash_pos.y);
            }
        }
    }
    fn solution_part_2(&mut self) -> String {
        loop {
            self.tick();
            if self.carts.is_empty() {
                panic!("no cart left!");
            }
            if self.carts.len() == 1 {
                let (last_cart_pos, _) = self.carts.first_key_value().unwrap();
                return format!("{},{}", last_cart_pos.x, last_cart_pos.y);
            }
        }
    }
    fn tick(&mut self) -> Vec<Point> {
        let mut new_carts: BTreeMap<Point, Cart> = BTreeMap::new();
        let mut crashes: Vec<Point> = Vec::new();
        while let Some((pos, cart)) = self.carts.pop_first() {
            let (new_pos, cart) = cart.tick(&pos, &self.cart_mine);
            if self.carts.contains_key(&new_pos) {
                self.carts.remove(&new_pos);
                crashes.push(new_pos);
                continue;
            }
            if new_carts.contains_key(&new_pos) {
                new_carts.remove(&new_pos);
                crashes.push(new_pos);
                continue;
            }
            new_carts.insert(new_pos, cart);
        }
        self.carts = new_carts;
        crashes
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_13.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_13 part 1: {result_part1}");
    assert_eq!(result_part1, "83,106");

    let result_part2 = challenge.solution_part_2();
    println!("result day_13 part 2: {result_part2}");
    assert_eq!(result_part2, "132,26");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_13() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_13_example_part_1.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_13 part 1: {result_part1}");
        assert_eq!(result_part1, "7,3");

        let input = include_str!("../../../../aoc_input/aoc-2018/day_13_example_part_2.txt");
        let mut example = ChallengeInput::from(input);
        let result_part2 = example.solution_part_2();
        println!("result day_13 part 2: {result_part2}");
        assert_eq!(result_part2, "6,4");

        Ok(())
    }
}
