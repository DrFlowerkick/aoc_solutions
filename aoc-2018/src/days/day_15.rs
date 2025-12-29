//!day_15.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default, Hash, PartialOrd, Ord)]
struct Unit {
    hit_points: u64,
    power: u64,
    /// true: elf, false: goblin
    race: bool,
}

impl Unit {
    fn new_elf() -> Self {
        Self {
            hit_points: 200,
            power: 3,
            race: true,
        }
    }
    fn new_goblin() -> Self {
        Self {
            hit_points: 200,
            power: 3,
            race: false,
        }
    }
    fn set_elf_power(mut self, power: u64) -> Self {
        if self.race {
            self.power = power;
        }
        self
    }
}

struct ChallengeInput {
    map: HashSet<Point>,
    initial_units: HashMap<Point, Unit>,
    units: BTreeMap<Point, Unit>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        let mut map = HashSet::new();
        let mut initial_units = HashMap::new();
        let mut units = BTreeMap::new();
        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Point::new(x as i64, y as i64);
                match c {
                    '.' => {
                        map.insert(pos);
                    }
                    'E' => {
                        map.insert(pos);
                        let unit = Unit::new_elf();
                        initial_units.insert(pos, unit);
                        units.insert(pos, unit);
                    }
                    'G' => {
                        map.insert(pos);
                        let unit = Unit::new_goblin();
                        initial_units.insert(pos, unit);
                        units.insert(pos, unit);
                    }
                    _ => (),
                }
            }
        }
        ChallengeInput {
            map,
            initial_units,
            units,
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self, stop_if_elf_dies: bool) -> u64 {
        let mut round_counter: u64 = 0;
        while !self.one_round_of_battle(stop_if_elf_dies) {
            round_counter += 1;
        }
        round_counter * self.units.values().map(|u| u.hit_points).sum::<u64>()
    }
    fn solution_part_2(&mut self) -> u64 {
        // get initial number of elf
        let initial_num_elves = self.initial_units.values().filter(|u| u.race).count();
        let mut min_power = 4;
        // use linear search, because problem is not monoton increasing
        loop {
            self.initialize_units(min_power);
            let outcome = self.solution_part_1(true);
            if initial_num_elves == self.units.values().filter(|u| u.race).count() {
                return outcome;
            }
            min_power += 1;
        }
    }
    fn initialize_units(&mut self, elf_power: u64) {
        self.units = self
            .initial_units
            .iter()
            .map(|(p, u)| (*p, u.set_elf_power(elf_power)))
            .collect();
    }
    fn one_round_of_battle(&mut self, stop_if_elf_dies: bool) -> bool {
        let mut resolved_units: BTreeMap<Point, Unit> = BTreeMap::new();
        while let Some((pos, unit)) = self.units.pop_first() {
            let opponents: HashSet<Point> = resolved_units
                .iter()
                .chain(self.units.iter())
                .filter_map(|(p, u)| (u.race != unit.race).then_some(*p))
                .collect();
            if opponents.is_empty() {
                // no more opponents -> battle is over
                self.units.insert(pos, unit);
                self.units.extend(resolved_units);
                return true;
            }
            let allies: HashSet<Point> = resolved_units
                .iter()
                .chain(self.units.iter())
                .filter_map(|(p, u)| (u.race == unit.race).then_some(*p))
                .collect();
            // If opponent is neighbor to unit, current pos of unit is returned as next_pos.
            // None is only returned, if there is no path to any target.
            if let Some((distance_to_target, next_pos)) =
                self.move_to_position(pos, &opponents, &allies)
            {
                // move
                resolved_units.insert(next_pos, unit);
                // check for attack:
                // if distance_to_target is 0, we are already at attack position
                // if distance is 1, we can attack after move
                if distance_to_target <= 1 {
                    // attack is possible, find targets from next_pos
                    // true value for 'resolved' means target is in resolved units, otherwise in self.units
                    if let Some(((opp_hit_points, opp_pos), resolved)) = self
                        .get_attack_targets(next_pos, !unit.race, &resolved_units)
                        .pop_first()
                    {
                        if opp_hit_points <= unit.power {
                            if resolved {
                                resolved_units.remove(&opp_pos);
                            } else {
                                self.units.remove(&opp_pos);
                            }
                            // kill opponent
                            if stop_if_elf_dies && !unit.race {
                                self.units.extend(resolved_units);
                                return true;
                            }
                        } else {
                            // attack opponent
                            let opp = if resolved {
                                resolved_units.get_mut(&opp_pos).unwrap()
                            } else {
                                self.units.get_mut(&opp_pos).unwrap()
                            };
                            opp.hit_points -= unit.power;
                        }
                    }
                }
            } else {
                // no position to move to, therefore no opponent to attack
                resolved_units.insert(pos, unit);
            }
        }
        self.units = resolved_units;
        false
    }
    fn move_to_position(
        &self,
        pos: Point,
        opponents: &HashSet<Point>,
        allies: &HashSet<Point>,
    ) -> Option<(u64, Point)> {
        // Note: current unit is not listed in opponents (obviously) or allies. Therefore current unit
        // does not block it's position in calc_distance().
        let targets: HashSet<Point> = opponents
            .iter()
            .flat_map(|p| Compass::cardinals().into_iter().map(|c| p.add(c)))
            .filter(|p| self.map.contains(p) && !opponents.contains(p) && !allies.contains(p))
            .collect();
        let distance_map_from_pos = self.calc_distance(pos, opponents, allies, &targets);
        // returns None, if no path to any target is available
        if let Some((distance, target)) = targets
            .iter()
            .filter_map(|p| distance_map_from_pos.get(p).map(|d| (*d, *p)))
            .collect::<BTreeSet<(u64, Point)>>()
            .pop_first()
        {
            // We include current pos with center. This is required if both units are neighbors.
            let next_positions: HashSet<Point> = Compass::center_and_cardinals()
                .into_iter()
                .map(|c| pos.add(c))
                .filter(|p| self.map.contains(p) && !opponents.contains(p) && !allies.contains(p))
                .collect();
            let distance_map_from_target =
                self.calc_distance(target, opponents, allies, &next_positions);
            if let Some((_, next_pos)) = next_positions
                .iter()
                .filter_map(|p| distance_map_from_target.get(p).map(|d| (*d, *p)))
                .collect::<BTreeSet<(u64, Point)>>()
                .pop_first()
            {
                return Some((distance, next_pos));
            }
        }
        None
    }
    fn calc_distance(
        &self,
        pos: Point,
        opponents: &HashSet<Point>,
        allies: &HashSet<Point>,
        targets: &HashSet<Point>,
    ) -> HashMap<Point, u64> {
        let mut seen: HashMap<Point, u64> = HashMap::new();
        let mut queue: VecDeque<(Point, u64)> = VecDeque::new();
        queue.push_back((pos, 0));

        // we only want shortest distance to targets
        let mut min_distance_to_target = u64::MAX;

        while let Some((current_pos, distance)) = queue.pop_front() {
            if distance > min_distance_to_target
                || seen.contains_key(&current_pos)
                || !self.map.contains(&current_pos)
                || opponents.contains(&current_pos)
                || allies.contains(&current_pos)
            {
                continue;
            }
            seen.insert(current_pos, distance);

            if targets.contains(&current_pos) {
                min_distance_to_target = distance;
            }

            // add next positions to queue
            for next in Compass::cardinals().into_iter().map(|c| current_pos.add(c)) {
                queue.push_back((next, distance + 1));
            }
        }
        seen
    }
    fn get_attack_targets(
        &self,
        pos: Point,
        opp_race: bool,
        resolved_units: &BTreeMap<Point, Unit>,
    ) -> BTreeMap<(u64, Point), bool> {
        // true value of BTreeMap: target is in resolved units, otherwise in self.units
        Compass::cardinals()
            .into_iter()
            .filter_map(|c| {
                let p = pos.add(c);
                if let Some(opp) = resolved_units.get(&p)
                    && opp.race == opp_race
                {
                    Some(((opp.hit_points, p), true))
                } else if let Some(opp) = self.units.get(&p)
                    && opp.race == opp_race
                {
                    Some(((opp.hit_points, p), false))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2018/day_15.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1(false);
    println!("result day_15 part 1: {result_part1}");
    assert_eq!(result_part1, 237_996);

    let result_part2 = challenge.solution_part_2();
    println!("result day_15 part 2: {result_part2}");
    assert_eq!(result_part2, 69_700);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_point_ordering() {
        let p1 = Point::new(1, 0);
        let p2 = Point::new(0, 1);
        let p3 = Point::new(1, 1);

        assert!(p1 < p2);
        assert!(p1 < p3);
        assert!(p2 < p3);

        let h1: u64 = 10;
        let h2: u64 = 10;
        let h3: u64 = 8;
        assert!((h1, p1) < (h2, p2));
        assert!((h3, p3) < (h1, p1));
    }

    #[test]
    fn test_example_day_15() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2018/day_15_example.txt");

        let solutions = [
            (27_730, 4_988),
            (36_334, 29_064), // solution for this riddle was not given
            (39_514, 31_284),
            (27_755, 3_478),
            (28_944, 6_474),
            (18_740, 1_140),
        ];

        for (ex, (solution_part_1, solution_part_2)) in input.split("\n\n").zip(solutions) {
            let mut example = ChallengeInput::from(ex);

            let result_part1 = example.solution_part_1(false);
            println!("result day_15 part 1: {result_part1}");
            assert_eq!(result_part1, solution_part_1);

            let result_part2 = example.solution_part_2();
            println!("result day_15 part 2: {result_part2}");
            assert_eq!(result_part2, solution_part_2);
        }

        Ok(())
    }
}
