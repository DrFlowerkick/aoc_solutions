//!day_19.rs

use super::day_05::IntCodeComputer;
use anyhow::Result;
use my_lib::my_geometry::{my_line::Line, my_point::Point};
use std::collections::HashMap;

struct ChallengeInput {
    code: IntCodeComputer,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            code: IntCodeComputer::from(value),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> Result<(usize, i64)> {
        let mut map: HashMap<Point, char> = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for y in 0..50 {
            for x in 0..50 {
                let val = if self.pos_is_pulled((x, y).into())? {
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    '#'
                } else {
                    '.'
                };
                map.insert((x, y).into(), val);
                // uncomment for dbg
                //print!("{val}");
            }
            // uncomment for dbg
            //println!();
        }

        // part 2
        let upper_beam_point = *map
            .iter()
            .filter_map(|(p, v)| (p.x == max_x && *v == '#').then_some(p))
            .min_by_key(|p| p.y)
            .unwrap();
        let lower_beam_point = *map
            .iter()
            .filter_map(|(p, v)| (p.y == max_y && *v == '#').then_some(p))
            .min_by_key(|p| p.x)
            .unwrap();
        let upper_beam = Line::from((Point::new(0, 0), upper_beam_point));
        let lower_beam = Line::from((Point::new(0, 0), lower_beam_point));
        let (m_ub, q_ub) = upper_beam.get_m_q().unwrap();
        let (m_lb, q_lb) = lower_beam.get_m_q().unwrap();

        // to start search of rec we start were beam starts be at least 99 wide on x-axis
        // -> find y_lb and x for lb, where x_lb and x_ub have 99 distance on x-axis
        // y_ub = m_ub * x + q_ub
        // y_lb = m_lb * x + q_lb
        // m_lb > m_ub !
        // y_lb(x) == y_ub(x + 99) !
        // m_lb * x + q_lb = m_ub * (x + 99) + q_ub
        // x * (m_lb - m_ub) = 99m_ub + q_ub - q_lb
        // x = (99m_ub + q_ub - q_lb) / (m_lb - m_ub)
        let left_rec = (99.0 * m_ub + q_ub - q_lb) / (m_lb - m_ub);
        let top_rec = m_lb * left_rec + q_lb;

        let mut top_left = Point::new(left_rec as i64, top_rec as i64);
        loop {
            let right = top_left.add((99, 0));
            let bottom = top_left.add((0, 99));
            if !self.pos_is_pulled(right)? {
                // move down
                top_left.y += 1;
            } else if !self.pos_is_pulled(bottom)? {
                // move right
                top_left.x += 1;
            } else {
                // found top_left
                break;
            }
        }
        Ok((
            map.values().filter(|v| **v == '#').count(),
            top_left.x * 10_000 + top_left.y,
        ))
    }
    fn pos_is_pulled(&self, pos: Point) -> Result<bool> {
        let mut parse_beam = self.code.clone();
        if let Some(is_pulled) = parse_beam
            .run_int_code(&[pos.x, pos.y])
            .map_err(|err| anyhow::anyhow!("{err}"))?
        {
            return Ok(is_pulled == 1);
        }
        Ok(false)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2019/day_19.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2()?;
    println!("result day_19 part 1: {result_part1}");
    assert_eq!(result_part1, 116);

    println!("result day_19 part 2: {result_part2}");
    assert_eq!(result_part2, 10_311_666);

    Ok(())
}

#[cfg(test)]
mod tests {
    /* int code challenge does not provide example
    use super::*;

    #[test]
    fn test_example_day_19() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2019/day_19_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_19 part 1: {result_part1}");
        //assert_eq!(result_part1, XXX);

        let result_part2 = example.solution_part_2();
        println!("result day_19 part 2: {result_part2}");
        //assert_eq!(result_part2, YYY);

        Ok(())
    }*/
}
