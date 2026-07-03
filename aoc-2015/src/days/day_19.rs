//!day_19.rs

use anyhow::Result;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

struct ChallengeInput<'a> {
    map: HashMap<&'a str, Vec<&'a str>>,
    molecule: &'a str,
}

impl<'a> From<&'a str> for ChallengeInput<'a> {
    fn from(value: &'a str) -> Self {
        let (map_str, molecule) = value.split_once("\n\n").unwrap();
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for (source, target) in map_str.lines().filter_map(|l| l.split_once(" => ")) {
            map.entry(source)
                .and_modify(|v| v.push(target))
                .or_insert(vec![target]);
        }
        ChallengeInput { map, molecule }
    }
}

impl<'a> ChallengeInput<'a> {
    fn solution_part_1(&self) -> usize {
        let mut seen: HashSet<String> = HashSet::new();
        self.recursive_collecting_of_transmutations("".into(), self.molecule, &mut seen);
        seen.len()
    }
    fn solution_part_2(&self) -> u64 {
        // Randomized Greedy Search
        // put replacement rules into random order and replace with first found rule until "e"
        // is found. If "e" is not found, try it with another randomly ordered rule set.
        let mut rng = rand::rng();
        // rule set: a list of replacement rules "reversed": first element in tuple is target, second is source
        let mut reverse_list = self.reverse_list();
        loop {
            reverse_list.shuffle(&mut rng);
            let mut count = 0;
            let mut molecule = String::from(self.molecule);
            while let Some((target, source)) =
                reverse_list.iter().find(|(t, _)| molecule.contains(t))
            {
                let (left, right) = molecule.split_once(target).unwrap();
                molecule = String::from(left) + source + right;
                count += 1;
                if molecule == "e" {
                    return count;
                }
            }
        }
    }
    fn recursive_collecting_of_transmutations(
        &self,
        left: String,
        right: &str,
        seen: &mut HashSet<String>,
    ) {
        if !right.is_empty() {
            if let Some(molecule) = self.map.keys().find(|k| right.starts_with(*k)) {
                let right = right.strip_prefix(molecule).unwrap();
                for transmutation in self.map.get(molecule).unwrap().iter() {
                    let new_molecule = left.clone() + transmutation + right;
                    seen.insert(new_molecule);
                }
                let left = left + molecule;
                self.recursive_collecting_of_transmutations(left, right, seen);
            } else {
                let left = left + &right[0..1];
                self.recursive_collecting_of_transmutations(left, &right[1..], seen);
            }
        }
    }
    fn reverse_list(&self) -> Vec<(&str, &str)> {
        // first assert unique transmutation targets
        let mut targets: HashSet<&str> = HashSet::new();
        for target in self.map.values().flat_map(|v| v.iter()) {
            assert!(targets.insert(*target));
        }
        // create reverse list
        self.map
            .iter()
            .flat_map(|(source, targets)| targets.iter().map(|target| (*target, *source)))
            .collect()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2015/day_19.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_19 part 1: {result_part1}");
    assert_eq!(result_part1, 576);

    let result_part2 = challenge.solution_part_2();
    println!("result day_19 part 2: {result_part2}");
    assert_eq!(result_part2, 207);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_19_part_1() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_19_example_part_1.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_19 part 1: {result_part1}");
        assert_eq!(result_part1, 4);

        Ok(())
    }

    #[test]
    fn test_example_day_19_part_2() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2015/day_19_example_part_2.txt");
        let example = ChallengeInput::from(input);

        let result_part2 = example.solution_part_2();
        println!("result day_19 part 2: {result_part2}");
        assert_eq!(result_part2, 6);

        Ok(())
    }
}
