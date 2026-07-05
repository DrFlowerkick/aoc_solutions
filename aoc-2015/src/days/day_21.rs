//!day_21.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone, Copy, Default, Debug)]
struct Item {
    cost: u64,
    damage: u64,
    armor: u64,
}

#[derive(Clone, Default, Debug)]
struct ItemShop<'a> {
    shop: HashMap<&'a str, HashMap<&'a str, Item>>,
}

impl<'a> From<&'a str> for ItemShop<'a> {
    fn from(value: &'a str) -> Self {
        ItemShop {
            shop: value
                .split("\n\n")
                .map(|item_list| {
                    let (header, list) = item_list.split_once("\n").unwrap();
                    let item_type = header.split_once(":").unwrap().0;
                    let items = list
                        .lines()
                        .map(|line| {
                            let mut item_iter = line.split_whitespace();
                            let item_name = item_iter.next().unwrap();
                            let item = Item {
                                cost: item_iter.next().unwrap().parse().unwrap(),
                                damage: item_iter.next().unwrap().parse().unwrap(),
                                armor: item_iter.next().unwrap().parse().unwrap(),
                            };
                            (item_name, item)
                        })
                        .collect();
                    (item_type, items)
                })
                .collect(),
        }
    }
}

impl<'a> ItemShop<'a> {
    fn add_no_armor_and_no_ring(&mut self) {
        self.shop
            .get_mut("Armor")
            .unwrap()
            .insert("No Armor", Item::default());
        self.shop
            .get_mut("Rings")
            .unwrap()
            .insert("No Ring", Item::default());
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Character {
    pub hit_points: u64,
    pub damage: u64,
    pub armor: u64,
    pub mana: u64,
}

impl From<&str> for Character {
    fn from(value: &str) -> Self {
        let mut attr_iter = value.lines();
        let hit_points = attr_iter
            .next()
            .and_then(|s| s.strip_prefix("Hit Points: "))
            .and_then(|v| v.parse().ok())
            .unwrap_or_default();
        let damage = attr_iter
            .next()
            .and_then(|s| s.strip_prefix("Damage: "))
            .and_then(|v| v.parse().ok())
            .unwrap_or_default();
        let armor = attr_iter
            .next()
            .and_then(|s| s.strip_prefix("Armor: "))
            .and_then(|v| v.parse().ok())
            .unwrap_or_default();

        Character {
            hit_points,
            damage,
            armor,
            mana: 0,
        }
    }
}

impl Character {
    fn equip_item(&mut self, item: &Item) -> u64 {
        self.damage += item.damage;
        self.armor += item.armor;
        item.cost
    }
    fn fight(&self, opp: &Character) -> bool {
        let self_turns_to_win = self.turns_to_win(opp);
        let opp_turns_to_win = opp.turns_to_win(self);
        self_turns_to_win <= opp_turns_to_win
    }
    fn turns_to_win(&self, opp: &Character) -> u64 {
        let damage = self.damage.saturating_sub(opp.armor).max(1);
        let turns_to_win = opp.hit_points / damage;
        if opp.hit_points.is_multiple_of(turns_to_win) {
            turns_to_win
        } else {
            turns_to_win + 1
        }
    }
}

struct ChallengeInput {
    me: Character,
    opp: Character,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            me: Character::default(),
            opp: Character::from(value),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self, item_shop: &ItemShop) -> (u64, u64) {
        let mut min_cost = u64::MAX;
        let mut max_cost = 0;
        // we require rings in fixed order
        let rings: Vec<Item> = item_shop
            .shop
            .get("Rings")
            .unwrap()
            .values()
            .copied()
            .collect();
        for weapon in item_shop.shop.get("Weapons").unwrap().values() {
            for armor in item_shop.shop.get("Armor").unwrap().values() {
                for (i, ring_1) in rings.iter().enumerate() {
                    for ring_2 in rings.iter().skip(i + 1) {
                        let mut me = self.me;
                        me.hit_points = 100;
                        let mut cost = me.equip_item(weapon);
                        // may be no armor
                        cost += me.equip_item(armor);

                        // try without any ring
                        if me.fight(&self.opp) {
                            min_cost = min_cost.min(cost);
                        } else {
                            max_cost = max_cost.max(cost);
                        }
                        // try with at least one ring
                        cost += me.equip_item(ring_1);
                        cost += me.equip_item(ring_2);
                        if me.fight(&self.opp) {
                            min_cost = min_cost.min(cost);
                        } else {
                            max_cost = max_cost.max(cost);
                        }
                    }
                }
            }
        }
        (min_cost, max_cost)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_21.txt");
    let challenge = ChallengeInput::from(input);

    let item_shop = include_str!("../../../../aoc_input/aoc-2015/day_21_items.txt");
    let mut item_shop = ItemShop::from(item_shop);
    item_shop.add_no_armor_and_no_ring();

    let (result_part1, result_part2) = challenge.solution_part_1_and_2(&item_shop);
    println!("result day_21 part 1: {result_part1}");
    assert_eq!(result_part1, 78);

    println!("result day_21 part 2: {result_part2}");
    assert_eq!(result_part2, 148);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fighting() {
        let me = Character {
            hit_points: 8,
            damage: 5,
            armor: 5,
            mana: 0,
        };
        let opp = Character {
            hit_points: 12,
            damage: 7,
            armor: 2,
            mana: 0,
        };
        assert!(me.fight(&opp));
    }
}
