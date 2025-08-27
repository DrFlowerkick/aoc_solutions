//!day_21.rs

use anyhow::Result;
use std::collections::HashMap;

type DiracCache = HashMap<ChallengeInput, (u64, u64)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    pos: u64,
    score: u64,
}

impl Player {
    fn new(pos: u64) -> Self {
        Player { pos, score: 0 }
    }
    fn do_steps(&mut self, steps: u64) -> u64 {
        self.pos = 1 + (self.pos - 1 + steps) % 10;
        self.score += self.pos;
        self.score
    }
}

struct DeterministicDice {
    current: u64,
    num_roles: u64,
}

impl DeterministicDice {
    fn tripple_roll(&mut self) -> u64 {
        self.num_roles += 3;
        [1; 3].into_iter().fold(0, |steps, delta| {
            self.current += delta;
            if self.current == 101 {
                self.current = 1;
            }
            self.current + steps
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ChallengeInput {
    p1: Player,
    p2: Player,
    active_player: bool,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let player_pos: Vec<u64> = value
            .lines()
            .filter_map(|l| l.split_once(": "))
            .filter_map(|(_, p)| p.parse::<u64>().ok())
            .collect();
        ChallengeInput {
            p1: Player::new(player_pos[0]),
            p2: Player::new(player_pos[1]),
            active_player: true,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        let mut active_player = self.active_player;
        let mut dice = DeterministicDice {
            current: 100,
            num_roles: 0,
        };
        let mut p1 = self.p1;
        let mut p2 = self.p2;
        loop {
            let steps = dice.tripple_roll();
            let new_score = if active_player {
                p1.do_steps(steps)
            } else {
                p2.do_steps(steps)
            };
            if new_score >= 1000 {
                break;
            }
            active_player = !active_player;
        }
        let loser_score = p1.score.min(p2.score);
        loser_score * dice.num_roles
    }
    fn solution_part_2(&self) -> u64 {
        let mut cache: DiracCache = HashMap::new();
        let (wins_p1, wins_p2) = self.run_dirac_match(0, 0, &mut cache);
        wins_p1.max(wins_p2)
    }
    fn run_dirac_match(&self, num_rolls: u64, steps: u64, cache: &mut DiracCache) -> (u64, u64) {
        if let Some(&(wins_p1, wins_p2)) = cache.get(self) {
            return (wins_p1, wins_p2);
        }

        let mut wins_p1 = 0;
        let mut wins_p2 = 0;
        if num_rolls < 3 {
            for next_steps in 1..=3 {
                let (wp_1, wp_2) = self.run_dirac_match(num_rolls + 1, steps + next_steps, cache);
                wins_p1 += wp_1;
                wins_p2 += wp_2;
            }
        } else {
            let mut next = *self;
            let (player, score) = if next.active_player {
                (&mut next.p1, (1, 0))
            } else {
                (&mut next.p2, (0, 1))
            };
            (wins_p1, wins_p2) = if player.do_steps(steps) >= 21 {
                // active player won
                score
            } else {
                next.active_player = !next.active_player;
                next.run_dirac_match(0, 0, cache)
            };
        }
        if num_rolls == 0 {
            cache.insert(*self, (wins_p1, wins_p2));
        }
        (wins_p1, wins_p2)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_21.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_21 part 1: {result_part1}");
    assert_eq!(result_part1, 925_605);

    let result_part2 = challenge.solution_part_2();
    println!("result day_21 part 2: {result_part2}");
    assert_eq!(result_part2, 486_638_407_378_784);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_21() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_21_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_21 part 1: {result_part1}");
        assert_eq!(result_part1, 739_785);

        let result_part2 = example.solution_part_2();
        println!("result day_21 part 2: {result_part2}");
        assert_eq!(result_part2, 444_356_092_776_315);

        Ok(())
    }
}
