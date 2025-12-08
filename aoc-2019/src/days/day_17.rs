//!day_17.rs

use super::day_05::IntCodeComputer;
use anyhow::Result;
use my_lib::my_geometry::my_point::{Point, Turns90};
use std::collections::HashMap;

struct ChallengeInput {
    code: IntCodeComputer,
    map: HashMap<Point, char>,
    bot: Point,
    direction: Point,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            code: IntCodeComputer::from(value),
            map: HashMap::new(),
            bot: Point::default(),
            direction: Point::default(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&mut self) -> i64 {
        if let Err(err) = self.parse_map() {
            panic!("{err}");
        }
        self.calc_alignment_parameters()
    }
    fn solution_part_2(&mut self) -> i64 {
        // enable robot
        self.code.set_address(0, 2);
        // get movement functions
        let mut int_code_input_string = self.get_movement_routine_and_functions();
        // we do not want enable video feed; set to y if you want continuous printing of map
        int_code_input_string.push("n\n".into());
        // generate input
        let mut inputs: Vec<Vec<i64>> = int_code_input_string
            .iter()
            .map(|i| i.chars().map(|c| (c as u8) as i64).collect())
            .collect();
        // we start with no input
        inputs.insert(0, vec![]);
        let mut index = 0;
        while let Some(out) = self
            .code
            .run_int_code(inputs.get(index).unwrap_or(&vec![]))
            .unwrap()
        {
            if out > 255 {
                // no ascii output -> final result!
                return out;
            } else {
                let ch = (out as u8) as char;
                // uncomment for print of map and more debug information
                //print!("{ch}");
                if ch == ':' || ch == '?' {
                    // received prompt for input
                    index += 1;
                }
            }
        }
        0
    }
    fn parse_map(&mut self) -> Result<(), String> {
        let mut parser = self.code.clone();
        let mut current = Point::new(0, 0);
        while let Some(ascii) = parser.run_int_code(&[])? {
            let ch = (ascii as u8) as char;
            // uncomment for print of map
            //print!("{ch}");
            if ch == '\n' {
                current.x = 0;
                current.y += 1;
            } else {
                self.map.insert(current, ch);
                if ['^', '<', '>', 'v'].contains(&ch) {
                    self.bot = current;
                    // positive y points down!
                    self.direction = match ch {
                        '^' => (0, -1).into(),
                        'v' => (0, 1).into(),
                        '<' => (-1, 0).into(),
                        '>' => (1, 0).into(),
                        _ => unreachable!(),
                    }
                }
                current.x += 1;
            }
        }
        Ok(())
    }
    fn calc_alignment_parameters(&self) -> i64 {
        self.map
            .iter()
            .filter(|(p, c)| {
                **c == '#' && {
                    [(0, 1), (1, 0), (0, -1), (-1, 0)]
                        .into_iter()
                        .map(|n| p.add(n))
                        .filter_map(|n| self.map.get(&n))
                        .all(|c| *c == '#')
                }
            })
            .map(|(p, _)| p.x * p.y)
            .sum()
    }
    fn get_movement_routine_and_functions(&mut self) -> Vec<String> {
        // looking at printout of map, I assume that
        // 1. the bot should always turn in corners and drive through intersections
        // 2. movement functions consists of pairs of a turn and step commands
        // 3. the number of steps taken is always > 0
        // 4. movement functions contain always more than one pair of turn and step commands
        // 5. every movement function starts with another pair of turn and step commands
        //
        // We start by walking the map and collect pairs of turn and step commands
        let mut pairs: Vec<String> = Vec::new();
        let mut turn = 'N';
        let mut steps = 0;
        loop {
            let next_pos = self.bot.add(self.direction);
            let next = *self.map.get(&next_pos).unwrap_or(&'.');
            if next == '.' {
                // reached a wall, at start steps are 0
                if steps > 0 {
                    // not at start: add new pair
                    let pair = format!("{turn},{steps}");
                    pairs.push(pair);
                    // reset pair parameters
                    turn = 'N';
                    steps = 0;
                }
                // we have to turn either left or right
                // since positiv y points down, clockwise is left
                let left = self.direction.turn(Turns90::T90, true);
                if let Some(ch) = self.map.get(&self.bot.add(left))
                    && *ch == '#'
                {
                    turn = 'L';
                    self.direction = left;
                }
                let right = self.direction.turn(Turns90::T90, false);
                if let Some(ch) = self.map.get(&self.bot.add(right))
                    && *ch == '#'
                {
                    turn = 'R';
                    self.direction = right;
                }
                if turn == 'N' {
                    // No turn possible. Reached and of scaffolds
                    break;
                }
            } else {
                // do one step
                steps += 1;
                self.bot = next_pos;
            }
        }
        // now that we have all pairs, let's identify patterns and therefore movement functions
        let mut pattern_search = pairs.clone();
        let mut movement_functions: Vec<Vec<String>> = Vec::new();
        let mut pattern: Vec<String> = Vec::new();
        let mut next_pattern_start = 0;
        let mut pos_1 = 0;
        let mut pos_2 = 0;
        while !pattern_search.is_empty() {
            if pattern.is_empty() {
                // start new pattern search
                let pair = pattern_search[0].clone();
                pos_2 = pattern_search
                    .iter()
                    .enumerate()
                    .position(|(i, p)| i > 0 && *p == pair)
                    .expect("could not find next pair");
                pattern.push(pair);
                // keep track of start of next pattern to prevent overlapping search indices
                next_pattern_start = pos_2;
                // set indices to next positions to compare
                pos_1 = 1;
                pos_2 += 1;
            } else if pos_1 != next_pattern_start && pattern_search[pos_1] == pattern_search[pos_2]
            {
                // found next pair of pattern
                pattern.push(pattern_search[pos_1].clone());
                pos_1 += 1;
                pos_2 += 1;
            } else {
                // pattern ended
                // remove pattern from pattern_search
                while let Some(start_pos) = pattern_search.iter().position(|p| *p == pattern[0]) {
                    let after_pattern_pos = start_pos + pattern.len();
                    // remove in reverse order to prevent invalid indices
                    for pos in (start_pos..after_pattern_pos).rev() {
                        pattern_search.remove(pos);
                    }
                }
                // add pattern to movement function
                movement_functions.push(pattern.clone());
                // reset pattern to enable search for next pattern
                pattern.clear();
            }
        }
        assert!(movement_functions.len() <= 3);
        // now we have the patterns, let's generate the main movement routine
        let mut pair_index = 0;
        let mut movement_routine: Vec<String> = Vec::new();
        let movement_function_names = ["A", "B", "C"];
        while pair_index < pairs.len() {
            let pattern_start_pair = &pairs[pair_index];
            let index_movement_function = movement_functions
                .iter()
                .position(|mf| mf[0] == *pattern_start_pair)
                .unwrap();
            movement_routine.push(movement_function_names[index_movement_function].into());
            pair_index += movement_functions[index_movement_function].len();
        }
        // collect all int code input sequences in separate Strings
        let mut int_code_input: Vec<String> = Vec::new();
        int_code_input.push(movement_routine.join(",") + "\n");
        for movement_function in movement_functions {
            int_code_input.push(movement_function.join(",") + "\n");
        }
        int_code_input
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_17.txt");
    let mut challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_17 part 1: {result_part1}");
    assert_eq!(result_part1, 2080);

    let result_part2 = challenge.solution_part_2();
    println!("result day_17 part 2: {result_part2}");
    assert_eq!(result_part2, 742_673);

    Ok(())
}

#[cfg(test)]
mod tests {
    // int code challenge does not provide challenge example
    /*use super::*;

    #[test]
    fn test_example_day_17() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_17_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_17 part 1: {result_part1}");
        //assert_eq!(result_part1, XXX);

        let result_part2 = example.solution_part_2();
        println!("result day_17 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }*/
}
