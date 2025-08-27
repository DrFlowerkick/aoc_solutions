//!day_16.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Day16Data<const N: usize> {
    map: MyMap2D<char, N, N>,
    start: MapPoint<N, N>,
    end: MapPoint<N, N>,
}

impl<const N: usize> From<&str> for Day16Data<N> {
    fn from(value: &str) -> Self {
        let map: MyMap2D<char, N, N> = MyMap2D::from(value);
        let start = map
            .iter()
            .find(|(_, c)| **c == 'S')
            .map(|(s, _)| s)
            .unwrap();
        let end = map
            .iter()
            .find(|(_, c)| **c == 'E')
            .map(|(s, _)| s)
            .unwrap();
        Self { map, start, end }
    }
}

impl<const N: usize> Day16Data<N> {
    fn get_min_score_and_num_best_path_tiles(&self) -> (usize, usize) {
        // part 1: walk from start to end and count score and steps
        let mut score_cache: HashMap<(MapPoint<N, N>, Compass), (usize, usize)> =
            HashMap::with_capacity(N);
        let mut visit: VecDeque<(MapPoint<N, N>, Compass, usize, usize)> = VecDeque::new();
        visit.push_back((self.start, Compass::E, 0, 0));
        let mut min_score: Option<usize> = None;
        while let Some((tile, direction, score, steps)) = visit.pop_front() {
            // check current score
            if let Some(end_score) = min_score {
                if score > end_score {
                    // current score is too expensive -> skip it
                    continue;
                }
            }
            // check cached score of path
            if let Some((cached_score, _)) = score_cache.get(&(tile, direction)) {
                if score > *cached_score {
                    // current path is more expensive than other path -> skip
                    continue;
                }
            }
            // insert or update score
            score_cache.insert((tile, direction), (score, steps));
            // check end
            if tile == self.end {
                // end of maze
                min_score = Some(score);
                continue;
            }
            for (turns, (neighbor, dir)) in tile
                .iter_neighbors(direction, false, false, false)
                .enumerate()
                .filter(|(t, (n, _))| *t != 2 && *self.map.get(*n) != '#')
            {
                let new_score = if turns == 0 { score + 1 } else { score + 1001 };
                visit.push_back((neighbor, dir, new_score, steps + 1));
            }
        }
        let min_score = min_score.unwrap();

        // part 2: walk in reverse step by step through the cache
        let mut best_path_tiles: HashSet<MapPoint<N, N>> = HashSet::new();
        let mut visit_backwards: VecDeque<(MapPoint<N, N>, usize, usize)> = score_cache
            .iter()
            .filter(|((p, _), (s, _))| *p == self.end && *s == min_score)
            .map(|((p, _), (sc, st))| (*p, *sc, *st))
            .collect();
        while let Some((tile, score, steps)) = visit_backwards.pop_front() {
            // insert in cache
            best_path_tiles.insert(tile);
            if tile == self.start {
                // reached started -> skip neighbor search
                continue;
            }
            // search possible neighbors, which are one step away, and get their cached score and steps
            for (neighbor, cached_score, cached_steps) in tile
                .iter_neighbors(Compass::N, false, false, false)
                .filter(|(n, _)| *self.map.get(*n) != '#')
                .flat_map(|(n, d)| {
                    [
                        d.flip(),
                        d.clockwise().clockwise(),
                        d.counterclockwise().counterclockwise(),
                    ]
                    .into_iter()
                    .map(move |nd| (n, nd))
                })
                .filter_map(|(n, d)| score_cache.get(&(n, d)).map(|(csc, cst)| (n, csc, cst)))
                .filter(|(_, csc, cst)| **cst == steps - 1 && **csc < score)
            {
                visit_backwards.push_back((neighbor, *cached_score, *cached_steps));
            }
        }
        (min_score, best_path_tiles.len())
    }
}

// taken from ../../../../aoc_input/aoc-2024/day_16.txt
const N: usize = 141;

pub fn day_16() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_16.txt");
    let challenge = Day16Data::<N>::from(input);

    let (result_part1, result_part2) = challenge.get_min_score_and_num_best_path_tiles();
    println!("result day 16 part 1: {}", result_part1);
    assert_eq!(result_part1, 83_432);

    println!("result day 16 part 2: {}", result_part2);
    assert_eq!(result_part2, 467);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    // taken from ../../../../aoc_input/aoc-2024/day_16_example_01.txt
    const E01: usize = 15;
    // taken from ../../../../aoc_input/aoc-2024/day_16_example_02.txt
    const E02: usize = 17;

    #[test]
    fn test_example_part() -> Result<()> {
        let input_01 = include_str!("../../../../aoc_input/aoc-2024/day_16_example_01.txt");
        let challenge_01 = Day16Data::<E01>::from(input_01);
        let (result_part1_1, result_part2_1) = challenge_01.get_min_score_and_num_best_path_tiles();
        println!("result day 16 part 1_1: {}", result_part1_1);
        assert_eq!(result_part1_1, 7_036);
        println!("result day 16 part 2_1: {}", result_part2_1);
        assert_eq!(result_part2_1, 45);

        let input_02 = include_str!("../../../../aoc_input/aoc-2024/day_16_example_02.txt");
        let challenge_02 = Day16Data::<E02>::from(input_02);
        let (result_part1_2, result_part2_2) = challenge_02.get_min_score_and_num_best_path_tiles();
        println!("result day 16 part 1_2: {}", result_part1_2);
        assert_eq!(result_part1_2, 11_048);
        println!("result day 16 part 2_2: {}", result_part2_2);
        assert_eq!(result_part2_2, 64);
        Ok(())
    }
}
