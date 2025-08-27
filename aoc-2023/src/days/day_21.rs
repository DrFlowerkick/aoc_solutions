//!day_21.rs

use anyhow::Result;
use my_lib::my_compass::Compass;
use my_lib::my_map_point::MapPoint;
use my_lib::my_map_two_dim::MyMap2D;

// values taken from ../../../../aoc_input/aoc-2023/day_21.txt
const X: usize = 131;
const Y: usize = 131;

struct Garden<const X: usize, const Y: usize> {
    map: MyMap2D<char, X, Y>,
}

impl<const X: usize, const Y: usize> Garden<X, Y> {
    fn from_str(input: &str) -> Self {
        Self {
            map: MyMap2D::from(input),
        }
    }
    fn count_steps(
        &self,
        start_point: MapPoint<X, Y>,
        steps_to_take: usize,
        count_even: bool,
    ) -> usize {
        let filter_fn = Box::new(
            move |_: MapPoint<X, Y>,
                  value_of_next_cell: &char,
                  _: Compass,
                  _: MapPoint<X, Y>,
                  _: &char,
                  current_distance: usize| {
                (*value_of_next_cell == '.' || *value_of_next_cell == 'S')
                    && current_distance <= steps_to_take
            },
        );
        let remainder = if count_even { 0 } else { 1 };
        let garden_tiles = self
            .map
            .iter_distance(start_point, filter_fn)
            .filter(|(.., d)| *d % 2 == remainder)
            .count();
        garden_tiles
    }
    fn count_steps_infinite_garden(&self, steps_to_take: usize) -> usize {
        // check S is in middle of
        let start_point = self
            .map
            .iter()
            .find(|(_, c)| **c == 'S')
            .map(|(p, _)| p)
            .unwrap();
        assert_eq!(start_point.x(), X / 2);
        assert_eq!(start_point.y(), Y / 2);
        // no rocks in center vertical or horizontal line, or at edge
        assert!(!self
            .map
            .iter_column(start_point.x())
            .any(|(_, v)| *v == '#'));
        assert!(!self.map.iter_row(start_point.y()).any(|(_, v)| *v == '#'));
        assert!(!self
            .map
            .iter_edge((0, 0).into(), false)
            .any(|(_, v)| *v == '#'));
        // calc grid size
        let grid_cells_one_direction = steps_to_take / X;
        let grid_cells_remaining_steps = steps_to_take % X;
        // steps you take from center garden to map edge
        let steps_to_edge = X / 2;
        assert_eq!(grid_cells_remaining_steps, steps_to_edge);
        let num_even_grids = grid_cells_one_direction.pow(2);
        let num_odd_grids = (grid_cells_one_direction - 1).pow(2);
        let steps_even_grid = self.count_steps(start_point, steps_to_take, true);
        let steps_odd_grid = self.count_steps(start_point, steps_to_take, false);
        // first count full grids
        let mut garden_tiles = steps_even_grid * num_even_grids + steps_odd_grid * num_odd_grids;
        // 4 corner tiles
        let steps_corner_n = self.count_steps((X / 2, Y - 1).into(), X - 1, true);
        let steps_corner_e = self.count_steps((0, Y / 2).into(), X - 1, true);
        let steps_corner_s = self.count_steps((X / 2, 0).into(), X - 1, true);
        let steps_corner_w = self.count_steps((X - 1, Y / 2).into(), X - 1, true);
        garden_tiles += steps_corner_n + steps_corner_e + steps_corner_s + steps_corner_w;
        // small side tiles
        let steps_small_side_tile_nw = self.count_steps((X - 1, Y - 1).into(), X / 2 - 1, true);
        let steps_small_side_tile_ne = self.count_steps((0, Y - 1).into(), X / 2 - 1, true);
        let steps_small_side_tile_sw = self.count_steps((X - 1, 0).into(), X / 2 - 1, true);
        let steps_small_side_tile_se = self.count_steps((0, 0).into(), X / 2 - 1, true);
        garden_tiles += (steps_small_side_tile_ne
            + steps_small_side_tile_nw
            + steps_small_side_tile_se
            + steps_small_side_tile_sw)
            * grid_cells_one_direction;
        // big side tiles
        let steps_big_side_tile_nw = self.count_steps((X - 1, Y - 1).into(), X + X / 2 - 1, false);
        let steps_big_side_tile_ne = self.count_steps((0, Y - 1).into(), X + X / 2 - 1, false);
        let steps_big_side_tile_sw = self.count_steps((X - 1, 0).into(), X + X / 2 - 1, false);
        let steps_big_side_tile_se = self.count_steps((0, 0).into(), X + X / 2 - 1, false);
        garden_tiles += (steps_big_side_tile_ne
            + steps_big_side_tile_nw
            + steps_big_side_tile_se
            + steps_big_side_tile_sw)
            * (grid_cells_one_direction - 1);
        garden_tiles
    }
}

pub fn day_21() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_21.txt");
    let garden = Garden::<X, Y>::from_str(input);
    let steps_to_take = 64;
    let start_point = garden
        .map
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(p, _)| p)
        .unwrap();
    let result_part1 = garden.count_steps(start_point, steps_to_take, true);
    println!("result day 21 part 1: {}", result_part1);
    assert_eq!(result_part1, 3_697);
    let steps_to_take = 26_501_365;
    let result_part2 = garden.count_steps_infinite_garden(steps_to_take);
    println!("result day 21 part 2: {}", result_part2);
    assert_eq!(result_part2, 608_152_828_731_262);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const XT: usize = 11;
    const YT: usize = 11;

    #[test]
    fn test_example_part1() -> Result<()> {
        let input = "...........\n\
                           .....###.#.\n\
                           .###.##..#.\n\
                           ..#.#...#..\n\
                           ....#.#....\n\
                           .##..S####.\n\
                           .##..#...#.\n\
                           .......##..\n\
                           .##.#.####.\n\
                           .##..##.##.\n\
                           ...........";
        let garden = Garden::<XT, YT>::from_str(input);
        let steps_to_take = 6;
        let start_point = garden
            .map
            .iter()
            .find(|(_, c)| **c == 'S')
            .map(|(p, _)| p)
            .unwrap();
        let result_part1 = garden.count_steps(start_point, steps_to_take, true);
        println!("result day 21 example part 1: {}", result_part1);
        assert_eq!(result_part1, 16);
        Ok(())
    }

    #[test]
    fn test_calculations_part2() {
        let num_steps: usize = 26_501_365;
        let grid_cells_one_direction = num_steps / X;
        println!("grid_cells_one_direction: {}", grid_cells_one_direction);
        let grid_cells_remaining_steps = num_steps % X;
        println!("grid_cells_remaining_steps: {}", grid_cells_remaining_steps);
        let steps_to_edge = X / 2;
        println!("steps_to_edge: {}", steps_to_edge);
        let num_even_grids = grid_cells_one_direction.pow(2);
        println!("num_even_grids: {}", num_even_grids);
        let num_odd_grids = (grid_cells_one_direction - 1).pow(2);
        println!("num_odd_grids: {}", num_odd_grids);
        let steps_along_side_from_middle_to_next_grid = X / 2 + 2;
        let remaining_steps = X - steps_along_side_from_middle_to_next_grid;
        let remaining_steps_2 = X / 2 - 1;
        assert_eq!(remaining_steps, remaining_steps_2);
    }
}
