//!day_22.rs

use super::day_21::Character;
use anyhow::Result;
use std::collections::{BTreeSet, HashSet};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ChallengeInput {
    me: Character,
    opp: Character,
    shield: Option<u64>,
    poison: Option<u64>,
    recharge: Option<u64>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            me: Character::default(),
            opp: Character::from(value),
            shield: None,
            poison: None,
            recharge: None,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut game_state = *self;
        game_state.me.hit_points = 50;
        game_state.me.mana = 500;
        game_state.magic_duel(false)
    }
    fn solution_part_2(&self) -> u64 {
        let mut game_state = *self;
        game_state.me.hit_points = 50;
        game_state.me.mana = 500;
        game_state.magic_duel(true)
    }
    fn magic_duel(&self, hard: bool) -> u64 {
        let mut seen: HashSet<(u64, ChallengeInput)> = HashSet::new();
        let mut sorted_queue: BTreeSet<(u64, ChallengeInput)> = BTreeSet::new();
        sorted_queue.insert((0, *self));
        while let Some((casted_mana, mut game_state)) = sorted_queue.pop_first() {
            if seen.insert((casted_mana, game_state)) {
                if game_state.apply_hard_mode(hard) {
                    continue;
                }
                game_state.apply_effects();

                let possible_next_game_states = game_state.apply_possible_spells(casted_mana);
                if possible_next_game_states.is_empty() {
                    continue;
                }
                for (next_casted_mana, mut next_game_state) in possible_next_game_states {
                    if next_game_state.opp.hit_points == 0 {
                        return next_casted_mana;
                    }

                    // hard mode before opp acts
                    if game_state.apply_hard_mode(hard) {
                        continue;
                    }

                    // apply effects before opp acts
                    next_game_state.apply_effects();
                    if next_game_state.opp.hit_points == 0 {
                        return next_casted_mana;
                    }
                    // opp acts
                    next_game_state.opp_attacks();

                    if next_game_state.me.hit_points > 0 {
                        sorted_queue.insert((next_casted_mana, next_game_state));
                    }
                }
            }
        }
        0
    }
    fn apply_effects(&mut self) {
        if let Some(mut count) = self.shield {
            count -= 1;
            if count == 0 {
                self.me.armor = 0;
                self.shield = None
            } else {
                self.me.armor = 7;
                self.shield = Some(count);
            }
        }
        if let Some(mut count) = self.poison {
            self.opp.hit_points = self.opp.hit_points.saturating_sub(3);
            count -= 1;
            if count == 0 {
                self.poison = None
            } else {
                self.poison = Some(count);
            }
        }
        if let Some(mut count) = self.recharge {
            self.me.mana += 101;
            count -= 1;
            if count == 0 {
                self.recharge = None
            } else {
                self.recharge = Some(count);
            }
        }
    }
    fn apply_possible_spells(&self, casted_mana: u64) -> Vec<(u64, ChallengeInput)> {
        let mut next_game_states = Vec::new();
        // Magic Missile
        if self.me.mana >= 53 {
            let mut next_game_state = *self;
            next_game_state.me.mana -= 53;
            next_game_state.opp.hit_points = next_game_state.opp.hit_points.saturating_sub(4);
            next_game_states.push((casted_mana + 53, next_game_state));
        }
        // Drain
        if self.me.mana >= 73 {
            let mut next_game_state = *self;
            next_game_state.me.mana -= 73;
            next_game_state.me.hit_points += 2;
            next_game_state.opp.hit_points = next_game_state.opp.hit_points.saturating_sub(2);
            next_game_states.push((casted_mana + 73, next_game_state));
        }
        // Shield
        if self.shield.is_none() && self.me.mana >= 113 {
            let mut next_game_state = *self;
            next_game_state.me.mana -= 113;
            next_game_state.shield = Some(6);
            next_game_states.push((casted_mana + 113, next_game_state));
        }
        // Poison
        if self.poison.is_none() && self.me.mana >= 173 {
            let mut next_game_state = *self;
            next_game_state.me.mana -= 173;
            next_game_state.poison = Some(6);
            next_game_states.push((casted_mana + 173, next_game_state));
        }
        // Recharge
        if self.recharge.is_none() && self.me.mana >= 229 {
            let mut next_game_state = *self;
            next_game_state.me.mana -= 229;
            next_game_state.recharge = Some(5);
            next_game_states.push((casted_mana + 229, next_game_state));
        }
        next_game_states
    }
    fn opp_attacks(&mut self) {
        let damage = self.opp.damage.saturating_sub(self.me.armor).max(1);
        self.me.hit_points = self.me.hit_points.saturating_sub(damage);
    }
    fn apply_hard_mode(&mut self, hard: bool) -> bool {
        if hard {
            self.me.hit_points -= 1;
            return self.me.hit_points == 0;
        }
        false
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_22.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_22 part 1: {result_part1}");
    assert_eq!(result_part1, 953);

    let result_part2 = challenge.solution_part_2();
    println!("result day_22 part 2: {result_part2}");
    assert_eq!(result_part2, 1_289);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_scenario_1() {
        let mut game_state = ChallengeInput::default();
        game_state.me.hit_points = 10;
        game_state.me.mana = 250;
        game_state.opp.hit_points = 13;
        game_state.opp.damage = 8;
        assert_eq!(game_state.magic_duel(false), 250 - 24);
    }

    #[test]
    fn test_scenario_2() {
        let mut game_state = ChallengeInput::default();
        game_state.me.hit_points = 10;
        game_state.me.mana = 250;
        game_state.opp.hit_points = 14;
        game_state.opp.damage = 8;
        assert_eq!(game_state.magic_duel(false), 250 + 5 * 101 - 114);
    }
}
