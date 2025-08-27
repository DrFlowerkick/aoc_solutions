//!day_23.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::{cmp::Ordering, collections::BTreeSet, fmt::Display};

type CellPoint = MapPoint<11, 5>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
enum Cell {
    Free,
    A,
    B,
    C,
    D,
    #[default]
    Blocked,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Free,
            'A' => Cell::A,
            'B' => Cell::B,
            'C' => Cell::C,
            'D' => Cell::D,
            _ => Cell::Blocked,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::A => 'A',
            Cell::B => 'B',
            Cell::C => 'C',
            Cell::D => 'D',
            Cell::Free => '.',
            Cell::Blocked => '#',
        };
        write!(f, "{c}")
    }
}

impl Cell {
    fn energy(&self) -> u64 {
        match self {
            Cell::A => 1,
            Cell::B => 10,
            Cell::C => 100,
            Cell::D => 1000,
            _ => 0,
        }
    }
    fn room_index(&self) -> Option<usize> {
        match self {
            Cell::A => Some(2),
            Cell::B => Some(4),
            Cell::C => Some(6),
            Cell::D => Some(8),
            _ => None,
        }
    }
    fn energy_index(&self) -> Option<usize> {
        match self {
            Cell::A => Some(3),
            Cell::B => Some(2),
            Cell::C => Some(1),
            Cell::D => Some(0),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct ChallengeInput {
    energy: [Option<u64>; 4],
    burrow: MyMap2D<Cell, 11, 5>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            energy: [None; 4],
            burrow: MyMap2D::from(value),
        }
    }
}

impl Ord for ChallengeInput {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare energy. index 0: D, index 3: A
        for (opt_se, opt_oe) in self.energy.iter().zip(other.energy.iter()) {
            match (opt_se, opt_oe) {
                (Some(s), Some(o)) => match s.cmp(o) {
                    Ordering::Equal => (),
                    cmp => return cmp,
                },
                (Some(_), None) => return Ordering::Less,
                (None, Some(_)) => return Ordering::Greater,
                (None, None) => (),
            }
        }
        // compare map -> if same energy, we need to differentiate both energy levels by state of map
        for ((_, s), (_, o)) in self.burrow.iter().zip(other.burrow.iter()) {
            match s.cmp(o) {
                Ordering::Equal => (),
                c => return c,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for ChallengeInput {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> u64 {
        self.solve()
    }
    fn solution_part_2(&self) -> u64 {
        self.solve()
    }
    fn solve(&self) -> u64 {
        let mut sorted_queue: BTreeSet<ChallengeInput> = BTreeSet::new();
        sorted_queue.insert(*self);
        //let mut max_counter = 0;
        while let Some(current) = sorted_queue.pop_first() {
            if current.is_sorted() {
                return current.get_energy();
            }
            // not final configuration -> make another move
            for (pos, amphipod) in current
                .burrow
                .iter()
                .filter(|(pos, _)| current.may_move(*pos))
            {
                let energy_per_step = amphipod.energy();
                for (target, steps) in current.scan_move_targets(pos).into_iter() {
                    let mut next = current;
                    next.burrow.set(pos, Cell::Free);
                    next.burrow.set(target, *amphipod);
                    next.add_energy(energy_per_step * steps as u64, amphipod);
                    sorted_queue.insert(next);
                }
            }
        }
        unreachable!()
    }
    fn get_energy(&self) -> u64 {
        self.energy.iter().filter_map(|e| *e).sum()
    }
    fn add_energy(&mut self, energy: u64, amphipod: &Cell) {
        let Some(index) = amphipod.energy_index() else {
            return;
        };
        self.energy[index] = if let Some(e) = self.energy[index] {
            Some(e + energy)
        } else {
            Some(energy)
        };
    }
    fn is_sorted(&self) -> bool {
        [Cell::A, Cell::B, Cell::C, Cell::D]
            .iter()
            .all(|a| self.is_room_sorted(a))
    }
    fn is_room_sorted(&self, amphipod: &Cell) -> bool {
        let Some(index) = amphipod.room_index() else {
            return false;
        };
        self.burrow
            .iter_column(index)
            .skip(1)
            .filter(|(_, a)| **a != Cell::Blocked)
            .all(|(_, a)| a == amphipod)
    }
    fn is_room_accessible(&self, amphipod: &Cell) -> bool {
        let Some(index) = amphipod.room_index() else {
            return false;
        };
        self.burrow
            .iter_column(index)
            .skip(1)
            .filter(|(_, a)| **a != Cell::Blocked)
            .all(|(_, a)| a == amphipod || *a == Cell::Free)
    }
    fn is_above_room(&self, pos: &CellPoint) -> bool {
        pos.y() == 0
            && [Cell::A, Cell::B, Cell::C, Cell::D]
                .iter()
                .filter_map(|a| a.room_index())
                .any(|x| x == pos.x())
    }
    fn may_move(&self, pos: CellPoint) -> bool {
        let amphipod = self.burrow.get(pos);
        let Some(index) = amphipod.room_index() else {
            return false;
        };
        if pos.y() == 0 {
            // amphipod is in hallway
            self.is_room_accessible(amphipod)
        } else if pos.x() == index {
            // amphipod is in his room
            self.burrow
                .iter_column(index)
                .skip(pos.y())
                .filter(|(_, a)| **a != Cell::Blocked)
                .any(|(_, a)| a != amphipod)
        } else {
            // amphipod is in other room
            true
        }
    }
    fn scan_move_targets(&self, pos: CellPoint) -> Vec<(CellPoint, usize)> {
        let amphipod = self.burrow.get(pos);
        let Some(index) = amphipod.room_index() else {
            return vec![];
        };
        let target_room_is_accessible = self.is_room_accessible(amphipod);
        let target_room = target_room_is_accessible.then_some({
            self.burrow.iter_column(index).skip(1).fold(
                CellPoint::new(index, 0),
                |free_pos, (p_a, a)| {
                    if *a == Cell::Free { p_a } else { free_pos }
                },
            )
        });
        let filter_fn = Box::new(
            move |point_of_next_cell: CellPoint,
                  value_of_next_cell: &Cell,
                  _orientation_of_next_cell: Compass,
                  _current_point: CellPoint,
                  _value_of_current_cell: &Cell,
                  _current_distance: usize| {
                *value_of_next_cell == Cell::Free
                    && (point_of_next_cell.y() == 0
                        || point_of_next_cell.x() == pos.x()
                        || (target_room_is_accessible && point_of_next_cell.x() == index))
            },
        );
        let targets: Vec<(CellPoint, usize)> = self
            .burrow
            .iter_distance(pos, filter_fn)
            .skip(1)
            .map(|(p, _, s)| (p, s))
            .collect();
        if let Some((room_pos, steps)) = targets.iter().find(|(t, _)| Some(*t) == target_room) {
            // can move directly to room
            return vec![(*room_pos, *steps)];
        }
        // check if in Hallway
        if pos.y() == 0 {
            // cannot move directly from Hallway to room -> ignore all other points when starting fom hallway
            return vec![];
        }
        // move to hallway -> only collect points with y == 0
        targets
            .into_iter()
            .filter(|(t, _)| t.y() == 0 && !self.is_above_room(t))
            .collect()
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_23.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_23 part 1: {result_part1}");
    assert_eq!(result_part1, 10_411);

    let input = include_str!("../../../../aoc_input/aoc-2021/day_23_part2.txt");
    let challenge = ChallengeInput::from(input);
    let result_part2 = challenge.solution_part_2();
    println!("result day_23 part 2: {result_part2}");
    assert_eq!(result_part2, 46_721);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_steps_of_example() {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_23_example_steps.txt");
        let example_steps: Vec<ChallengeInput> =
            input.split("\n\n").map(ChallengeInput::from).collect();

        let mut possible_next_steps: Vec<ChallengeInput> = Vec::new();
        let mut example_step_with_energy = example_steps[0];
        for example_step in example_steps.iter() {
            println!("example_step\n{}", example_step.burrow);
            example_step_with_energy = if !possible_next_steps.is_empty() {
                if let Some(les) = possible_next_steps
                    .iter()
                    .find(|pns| pns.burrow == example_step.burrow)
                {
                    *les
                } else {
                    println!("last state\n{}", example_step_with_energy.burrow);
                    panic!("could not find example_step in possible_next_steps");
                }
            } else {
                *example_step
            };
            if example_step_with_energy.is_sorted() {
                assert_eq!(example_step_with_energy.get_energy(), 12_521);
                break;
            }
            // not final configuration -> make another move
            possible_next_steps.clear();
            for (pos, amphipod) in example_step_with_energy
                .burrow
                .iter()
                .filter(|(pos, _)| example_step_with_energy.may_move(*pos))
            {
                let energy_per_step = amphipod.energy();
                for (target, steps) in example_step_with_energy.scan_move_targets(pos).into_iter() {
                    let mut next = example_step_with_energy;
                    next.burrow.set(pos, Cell::Free);
                    next.burrow.set(target, *amphipod);
                    next.add_energy(energy_per_step * steps as u64, amphipod);
                    println!("amphipod: {amphipod} on {pos}");
                    println!("target: {target}");
                    possible_next_steps.push(next);
                }
            }
        }
    }

    #[test]
    fn test_example_day_23() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_23_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_23 part 1: {result_part1}");
        assert_eq!(result_part1, 12_521);

        let input = include_str!("../../../../aoc_input/aoc-2021/day_23_example_part2.txt");
        let example = ChallengeInput::from(input);
        let result_part2 = example.solution_part_2();
        println!("result day_23 part 2: {result_part2}");
        assert_eq!(result_part2, 44_169);

        Ok(())
    }
}
