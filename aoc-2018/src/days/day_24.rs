//!day_24.rs

use anyhow::Result;
use regex::Regex;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone)]
struct Units {
    members: u64,
    hit_points_per_member: u64,
    immunities: String,
    weakness: String,
    damage_per_member: u64,
    damage_type: String,
    initiative: u64,
}

impl From<&str> for Units {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"^(\d+) units each with (\d+) hit points(?: \(([^)]*)\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
        let caps = re.captures(value).unwrap();
        // caps[1]: units
        // caps[2]: hit points
        // caps[3]: immunities/weaknesses
        // caps[4]: attack damage
        // caps[5]: attack type
        // caps[6]: initiative

        let mut units = Units {
            members: caps[1].parse().unwrap(),
            hit_points_per_member: caps[2].parse().unwrap(),
            immunities: "".into(),
            weakness: "".into(),
            damage_per_member: caps[4].parse().unwrap(),
            damage_type: caps[5].to_string(),
            initiative: caps[6].parse().unwrap(),
        };

        if let Some(immunities_weaknesses_match) = caps.get(3) {
            let immunities_weaknesses_str = immunities_weaknesses_match.as_str();
            for part in immunities_weaknesses_str.split(';') {
                let part = part.trim();
                if let Some(imm_list) = part.strip_prefix("immune to ") {
                    units.immunities = imm_list.to_string();
                } else if let Some(weak_list) = part.strip_prefix("weak to ") {
                    units.weakness = weak_list.to_string();
                }
            }
        }
        units
    }
}

impl Units {
    fn effective_power(&self) -> u64 {
        self.members * self.damage_per_member
    }
    fn calc_damage(&self, other: &Units) -> Option<u64> {
        if self.members == 0 || other.immunities.contains(&self.damage_type) {
            None
        } else if other.weakness.contains(&self.damage_type) {
            Some(self.effective_power() * 2)
        } else {
            Some(self.effective_power())
        }
    }
    fn apply_damage(&mut self, damage: u64) -> u64 {
        let kill_potential = damage / self.hit_points_per_member;
        let killed_members = if kill_potential > self.members {
            self.members
        } else {
            kill_potential
        };
        self.members -= killed_members;
        killed_members
    }
    fn boost(&mut self, boost: u64) {
        self.damage_per_member += boost;
    }
}

#[derive(Debug, Clone)]
struct ChallengeInput {
    immune_system: Vec<Units>,
    infections: Vec<Units>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let (immune_system, infections) = value.split_once("\n\n").unwrap();

        ChallengeInput {
            immune_system: immune_system.lines().skip(1).map(Units::from).collect(),
            infections: infections.lines().skip(1).map(Units::from).collect(),
        }
    }
}

enum BattleResult {
    ImmuneSystemWins(u64),
    Stalemate,
    InfectionsWin(u64),
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> u64 {
        // clone self to keep original armies
        let mut battle_units = self.clone();
        match battle_units.battle() {
            BattleResult::ImmuneSystemWins(remaining_units)
            | BattleResult::InfectionsWin(remaining_units) => remaining_units,
            BattleResult::Stalemate => panic!("should be a winner"),
        }
    }
    fn solution_part_2(&self) -> u64 {
        // binary search of boost
        let mut min_boost = 0;
        let mut max_boost = 100_000;
        let mut max_result = 0;
        loop {
            let mean_boost = (max_boost + min_boost) / 2;
            // clone self to keep original armies
            let mut battle_units = self.clone();
            battle_units.boost(mean_boost);
            match battle_units.battle() {
                BattleResult::ImmuneSystemWins(mean_result) => {
                    if mean_boost + 1 == max_boost {
                        // found solution
                        return mean_result;
                    }
                    max_result = mean_result;
                    max_boost = mean_boost;
                }
                BattleResult::InfectionsWin(_) | BattleResult::Stalemate => {
                    if mean_boost + 1 == max_boost {
                        // cannot reduce further -> return last max_result
                        return max_result;
                    }
                    min_boost = mean_boost;
                }
            }
        }
    }
    fn battle(&mut self) -> BattleResult {
        while !self.immune_system.is_empty() && !self.infections.is_empty() {
            let attack_order = self.target_phase();
            let killed_units = self.attack_phase(attack_order);
            // stalemate?
            if killed_units == 0 {
                return BattleResult::Stalemate;
            }
        }
        if self.immune_system.is_empty() {
            BattleResult::InfectionsWin(self.infections.iter().map(|u| u.members).sum())
        } else {
            BattleResult::ImmuneSystemWins(self.immune_system.iter().map(|u| u.members).sum())
        }
    }
    fn target_phase(&self) -> BTreeMap<u64, (bool, usize, usize)> {
        let mut targeting = BTreeMap::new();
        for army in [true, false] {
            let mut seen: HashSet<usize> = HashSet::new();
            let (attackers, defenders) = if army {
                (&self.immune_system, &self.infections)
            } else {
                (&self.infections, &self.immune_system)
            };
            // decreasing order: attacker effective power, than attacker initiative
            let mut attack_order: BTreeMap<(u64, u64), (&Units, usize)> = attackers
                .iter()
                .enumerate()
                .map(|(index, u)| ((u.effective_power(), u.initiative), (u, index)))
                .collect();
            while let Some((
                (_attacker_effective_power, attacker_initiative),
                (attacker, attacker_index),
            )) = attack_order.pop_last()
            {
                // decreasing order: potential damage to target, than defender effective power, than defender initiative
                let mut defender_to_attack: BTreeMap<(u64, u64, u64), usize> = defenders
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !seen.contains(i))
                    .filter_map(|(defender_index, defender)| {
                        attacker.calc_damage(defender).map(|damage| {
                            (
                                (damage, defender.effective_power(), defender.initiative),
                                defender_index,
                            )
                        })
                    })
                    .collect();
                if let Some((_, defender_index)) = defender_to_attack.pop_last() {
                    seen.insert(defender_index);
                    targeting.insert(attacker_initiative, (army, attacker_index, defender_index));
                }
            }
        }
        targeting
    }
    fn attack_phase(&mut self, mut attack_order: BTreeMap<u64, (bool, usize, usize)>) -> u64 {
        // count killed units. If no units die, we have a stalemate.
        let mut killed_units = 0;
        // decreasing order: initiative
        while let Some((_, (army, attacker_index, defender_index))) = attack_order.pop_last() {
            // we have to calculate attack damage "at real time", because already attacked units deal less damage,
            // since they lost some (or even all) members
            let (attacker, defender) = if army {
                (
                    &self.immune_system[attacker_index],
                    &mut self.infections[defender_index],
                )
            } else {
                (
                    &self.infections[attacker_index],
                    &mut self.immune_system[defender_index],
                )
            };
            if let Some(damage) = attacker.calc_damage(defender) {
                killed_units += defender.apply_damage(damage);
            }
        }
        self.immune_system = self
            .immune_system
            .iter()
            .filter(|u| u.members > 0)
            .cloned()
            .collect();
        self.infections = self
            .infections
            .iter()
            .filter(|u| u.members > 0)
            .cloned()
            .collect();
        killed_units
    }
    fn boost(&mut self, boost: u64) {
        self.immune_system.iter_mut().for_each(|u| u.boost(boost));
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_24.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_24 part 1: {result_part1}");
    assert_eq!(result_part1, 21_765);

    let result_part2 = challenge.solution_part_2();
    println!("result day_24 part 2: {result_part2}");
    assert_eq!(result_part2, 5_522);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_24() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_24_example.txt");
        let mut example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_24 part 1: {result_part1}");
        assert_eq!(result_part1, 5_216);

        let result_part2 = example.solution_part_2();
        println!("result day_24 part 2: {result_part2}");
        assert_eq!(result_part2, 51);

        Ok(())
    }
}
