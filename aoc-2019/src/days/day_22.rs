//!day_22.rs

use anyhow::Result;
use my_lib::my_algo_collection::egcd_i128;
use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

enum Technique {
    DealIntoNewStack,
    CutNCards(i128),
    DealWithIncrement(i128),
}

impl From<&str> for Technique {
    fn from(value: &str) -> Self {
        if value == "deal into new stack" {
            Self::DealIntoNewStack
        } else if let Some(cut) = value.strip_prefix("cut ") {
            Self::CutNCards(cut.parse().unwrap())
        } else {
            let increment = value.strip_prefix("deal with increment ").unwrap();
            Self::DealWithIncrement(increment.parse().unwrap())
        }
    }
}

impl Technique {
    fn reverse_pos(
        &self,
        size: i128,
        pos: i128,
        pos_factor: i128,
        pos_offset: i128,
    ) -> (i128, i128, i128) {
        match self {
            Self::DealIntoNewStack => (size - pos - 1, -pos_factor, size - pos_offset - 1),
            Self::CutNCards(cut) => ((pos + cut).rem_euclid(size), pos_factor, pos_offset + cut),
            Self::DealWithIncrement(inc) => {
                let (gcd, inv_candidate, _) = egcd_i128(inc.rem_euclid(size), size);
                assert_eq!(gcd, 1);
                let inv = inv_candidate.rem_euclid(size);
                (
                    (pos * inv).rem_euclid(size),
                    (pos_factor * inv).rem_euclid(size),
                    (pos_offset * inv).rem_euclid(size),
                )
            }
        }
    }
}

struct Card {
    value: i128,
    right: RefCell<Option<Rc<Card>>>,
    left: RefCell<Option<Rc<Card>>>,
}

impl Card {
    fn new(value: i128) -> Rc<Card> {
        Rc::new(Card {
            value,
            right: RefCell::new(None),
            left: RefCell::new(None),
        })
    }
    fn next(&self, direction: bool) -> Rc<Card> {
        if direction { self.right() } else { self.left() }
    }
    fn previous(&self, direction: bool) -> Rc<Card> {
        if direction { self.left() } else { self.right() }
    }
    fn right(&self) -> Rc<Card> {
        self.right.borrow().as_ref().unwrap().clone()
    }
    fn left(&self) -> Rc<Card> {
        self.left.borrow().as_ref().unwrap().clone()
    }
    fn relink_right(&self, link_target: Rc<Card>) -> Rc<Card> {
        *self.right.borrow_mut() = Some(link_target.clone());
        link_target
    }
    fn relink_left(&self, link_target: Rc<Card>) -> Rc<Card> {
        *self.left.borrow_mut() = Some(link_target.clone());
        link_target
    }
    fn relink_next(&self, link_target: Rc<Card>, direction: bool) -> Rc<Card> {
        if direction {
            self.relink_right(link_target)
        } else {
            self.relink_left(link_target)
        }
    }
    fn relink_previous(&self, link_target: Rc<Card>, direction: bool) -> Rc<Card> {
        if direction {
            self.relink_left(link_target)
        } else {
            self.relink_right(link_target)
        }
    }
}

struct DeckOfCards {
    top: Rc<Card>,
    // true: right, false: left
    direction: bool,
    size: i128,
}

impl DeckOfCards {
    fn new(n_cards: i128) -> Self {
        let inverted = n_cards < 0;
        let top = Card::new(0);
        let mut last = top.clone();
        for mut value in 1..n_cards.abs() {
            if inverted {
                value = -value;
            }
            let next = Card::new(value);
            last.relink_right(next.clone());
            next.relink_left(last.clone());
            last = next;
        }
        last.relink_right(top.clone());
        top.relink_left(last.clone());
        DeckOfCards {
            top,
            direction: true,
            size: n_cards,
        }
    }
    fn deal_into_new_stack(&mut self) {
        // reverse order
        self.direction = !self.direction;
        self.top = self.top.next(self.direction);
    }
    fn cut_n_cards(&mut self, cut: i128) {
        if cut > 0 {
            for _ in 0..cut {
                self.top = self.top.next(self.direction);
            }
        } else {
            for _ in 0..cut.abs() {
                self.top = self.top.previous(self.direction);
            }
        }
    }
    fn deal_with_increment(&self, inc: i128) {
        let mut previous: HashMap<i128, (i128, Rc<Card>)> =
            HashMap::with_capacity(self.size as usize);
        let mut next: HashMap<i128, (i128, Rc<Card>)> = HashMap::with_capacity(self.size as usize);
        let mut pos = 0;
        let mut current = self.top.clone();
        while pos < self.size {
            let next_card = current.next(self.direction);
            let (new_previous_pos, new_next_pos) =
                self.get_previous_and_next_pos_after_inc(pos, inc);
            if let Some((prev_pos, card)) = previous.get(&pos) {
                assert!(*prev_pos < pos);
                assert_eq!(*prev_pos, new_previous_pos);
                current.relink_previous(card.clone(), self.direction);
                card.relink_next(current.clone(), self.direction);
            }
            if let Some((next_pos, card)) = next.get(&pos) {
                assert!(*next_pos < pos);
                assert_eq!(*next_pos, new_next_pos);
                current.relink_next(card.clone(), self.direction);
                card.relink_previous(current.clone(), self.direction);
            }
            // pos is previous for new_next_pos
            previous.insert(new_next_pos, (pos, current.clone()));
            // pos is next for new_previous_pos
            next.insert(new_previous_pos, (pos, current.clone()));
            // increment for next cycle
            pos += 1;
            current = next_card;
        }
    }
    fn get_previous_and_next_pos_after_inc(&self, current_pos: i128, inc: i128) -> (i128, i128) {
        // the formula for new position after inc is inc_pos = (current_pos * inc) % size
        // we want to link to new previous and new next of current_pos, therefore we need to
        // identify each pos value
        // get inverse of inc mod size
        let (gcd, inv_candidate, _) = egcd_i128(inc.rem_euclid(self.size), self.size);
        assert_eq!(gcd, 1);
        let inv = inv_candidate.rem_euclid(self.size);
        // get inc_pos of current_pos
        let inc_pos = (current_pos * inc).rem_euclid(self.size);
        // 2. get one pos before inc_pos
        let prev_inc_pos = (inc_pos - 1).rem_euclid(self.size);
        // 3. get corresponding current pos to prev_inc_pos
        let new_previous_pos = (prev_inc_pos * inv).rem_euclid(self.size);
        // 4. get one pos after inc_pos
        let next_inc_pos = (inc_pos + 1).rem_euclid(self.size);
        // 5. get corresponding current pos to next_inc_pos
        let new_next_pos = (next_inc_pos * inv).rem_euclid(self.size);
        (new_previous_pos, new_next_pos)
    }
    fn iter_deck(&self) -> impl Iterator<Item = (i128, Rc<Card>)> {
        IterDeck {
            card: self.top.clone(),
            pos: 0,
            direction: self.direction,
            size: self.size,
        }
    }
}

struct IterDeck {
    card: Rc<Card>,
    pos: i128,
    direction: bool,
    size: i128,
}

impl Iterator for IterDeck {
    type Item = (i128, Rc<Card>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.size {
            return None;
        }
        let out = Some((self.pos, self.card.clone()));
        self.pos += 1;
        self.card = self.card.next(self.direction);
        out
    }
}

struct ChallengeInput {
    techniques: Vec<Technique>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            techniques: value.lines().map(Technique::from).collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i128 {
        let mut deck = DeckOfCards::new(10_007);
        self.shuffle_cards(&mut deck);
        deck.iter_deck().find(|(_, c)| c.value == 2019).unwrap().0
    }
    fn solution_part_2(&self) -> i128 {
        // Since we want to know, which card is at the end on pos 2020
        // we can solve this by starting at pos 2020 and check, which position this card
        // had before the last technique. And with this new position we do it again, and again, and again
        // until we are at the start. And since the final (or initial) position is equal to the
        // value of the card, we have now the solution of part 2.
        let mut pos = 2020;
        let size = 119_315_717_514_047;
        let shuffles = 101_741_582_076_661_i128;
        let mut last = 2020;
        let mut last_pos_parameters: Option<(i128, i128)> = None;
        // Applying all techniques to the deck results in a shift of postion. We can combine
        // all techniques into one function:
        // new_pos = (pos * pos_factor + pos_offset) % size
        // So let's identify these pos parameters
        let mut pos_factor = 1;
        let mut pos_offset = 0;
        for _shuffle_index in 0..2 {
            pos_factor = 1;
            pos_offset = 0;
            for technique in self.techniques.iter().rev() {
                (pos, pos_factor, pos_offset) =
                    technique.reverse_pos(size, pos, pos_factor, pos_offset);
            }
            pos_factor = pos_factor.rem_euclid(size);
            pos_offset = pos_offset.rem_euclid(size);
            if let Some((last_pos_factor, last_pos_offset)) = last_pos_parameters {
                assert_eq!(pos_factor, last_pos_factor);
                assert_eq!(pos_offset, last_pos_offset);
            }
            last_pos_parameters = Some((pos_factor, pos_offset));
            assert_eq!(pos, (pos_factor * last + pos_offset).rem_euclid(size));
            last = pos;
        }

        // Now we have pos parameters. These pos parameters have to be applied again
        // on the new_pos, and again, and again for a number of "shuffle" times.
        // This reapplying looks like this (without modulo):
        // 1) new_pos = pos * pos_factor + pos_offset
        // 2.) new_pos = pos * pos_factor^2 + pos_factor*pos_offset + pos_offset
        // 3.) new_pos = pos * pos_factor^3 + pos_factor^2*pos_offset + pos_factor*pos_offset + pos_offset
        // There is this formula for combining the pos_factor^i*pos_offset elements:
        // (Sum over i = 0..=n) of b * a^i = b * (a^(n-1) - 1) / (a - 1)
        // We have to integrate modulo size into this. I'm not sure, why this is correct,
        // but this is the solution:
        // new_pos = (pos * modpow(pos_factor, shuffles, size) +
        //            pos_offset * (modpow(pos_factor, shuffles, size) - 1) * modinv(pos_factor - 1, size)) % size
        // Since we work with big numbers and in the second part of the above formula we multiply
        // three big numbers, i128 is not sufficient for this task.
        // Therefore we use num-bigint::BigUint
        let start_pos = BigUint::from_i32(2020).unwrap();
        let pos_factor = BigUint::from_i128(pos_factor).unwrap();
        let pos_offset = BigUint::from_i128(pos_offset).unwrap();
        let shuffles = BigUint::from_i128(shuffles).unwrap();
        let size = BigUint::from_i128(size).unwrap();
        let one = BigUint::from_i32(1).unwrap();

        let pos = (start_pos * pos_factor.modpow(&shuffles, &size)
            + pos_offset
                * (pos_factor.modpow(&shuffles, &size) - one.clone())
                * (pos_factor - one).modinv(&size).unwrap())
            % size;

        let pos: i128 = pos.to_i128().unwrap();

        pos
    }
    fn shuffle_cards(&self, deck: &mut DeckOfCards) {
        for technique in self.techniques.iter() {
            match technique {
                Technique::DealIntoNewStack => deck.deal_into_new_stack(),
                Technique::CutNCards(cut) => deck.cut_n_cards(*cut),
                Technique::DealWithIncrement(inc) => deck.deal_with_increment(*inc),
            }
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_22.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_22 part 1: {result_part1}");
    assert_eq!(result_part1, 3_036);

    let result_part2 = challenge.solution_part_2();
    println!("result day_22 part 2: {result_part2}");
    assert_eq!(result_part2, 70_618_172_909_245);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deal_into_new_stack() {
        let mut deck = DeckOfCards::new(10);
        deck.deal_into_new_stack();
        assert_eq!(
            deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>(),
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
    }

    #[test]
    fn cut() {
        let mut deck = DeckOfCards::new(10);
        deck.cut_n_cards(3);
        assert_eq!(
            deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>(),
            [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
        );

        let mut deck = DeckOfCards::new(10);
        deck.cut_n_cards(-4);
        assert_eq!(
            deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>(),
            [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_deal_with_increment_v2() {
        let deck = DeckOfCards::new(10);
        deck.deal_with_increment(3);
        assert_eq!(
            deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>(),
            [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
        );
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = DeckOfCards::new(10);
        deck.deal_with_increment(3);
        assert_eq!(
            deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>(),
            [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
        );
        deck.deal_into_new_stack();
        assert_eq!(
            deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>(),
            [3, 6, 9, 2, 5, 8, 1, 4, 7, 0]
        );

        let mut deck = DeckOfCards::new(10);
        deck.deal_with_increment(7);
        assert_eq!(
            deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>(),
            [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
        );
        deck.deal_with_increment(9);
        assert_eq!(
            deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>(),
            [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
        );
        deck.cut_n_cards(-2);
        assert_eq!(
            deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>(),
            [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]
        );
    }

    #[test]
    fn test_example_day_22() -> Result<()> {
        let multi_input = include_str!("../../../../aoc_input/aoc-2019/day_22_example.txt");
        for example in multi_input.split("\n\n") {
            let (input, solution) = example.split_once("\nResult: ").unwrap();
            let solution: Vec<i128> = solution
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            let example = ChallengeInput::from(input);
            let mut deck = DeckOfCards::new(10);

            example.shuffle_cards(&mut deck);
            let result_part1 = deck.iter_deck().map(|(_, c)| c.value).collect::<Vec<_>>();
            println!("result day_22 part 1: {:?}", result_part1);
            assert_eq!(result_part1, solution);
        }

        Ok(())
    }

    #[test]
    fn test_part_two_with_values_from_part_1() {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_22.txt");
        let challenge = ChallengeInput::from(input);

        // position of card 2019 after on shuffle of 10_007 cards
        let old_pos = 3036;
        let mut pos = 3036;
        let size = 10_007;
        let mut pos_factor = 1;
        let mut pos_offset = 0;

        for technique in challenge.techniques.iter().rev() {
            (pos, pos_factor, pos_offset) =
                technique.reverse_pos(size, pos, pos_factor, pos_offset);
        }

        assert_eq!(pos, 2019);
        pos_factor = pos_factor.rem_euclid(size);
        pos_offset = pos_offset.rem_euclid(size);
        assert_eq!(pos, (pos_factor * old_pos + pos_offset).rem_euclid(size));
    }
}
