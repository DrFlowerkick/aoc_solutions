//!day_20.rs

use anyhow::Result;
use my_lib::{my_map_point::MapPoint, my_map_two_dim::MyMap2D};

#[derive(Debug, Clone)]
struct SeaMonster<const X: usize, const Y: usize> {
    offsets: Vec<(usize, usize)>,
}

impl<const X: usize, const Y: usize> SeaMonster<X, Y> {
    fn new() -> Self {
        SeaMonster {
            // X: 20, Y: 3: size of initial sea monster, if rotate, X and Y switch values
            offsets: include_str!("../../../../aoc_input/aoc-2020/day_20_sea_monster.txt")
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
                })
                .collect(),
        }
    }
    fn rotate_clockwise(&self) -> Self {
        Self {
            offsets: self
                .offsets
                .iter()
                .map(|sm| MapPoint::<X, Y>::from(*sm).rotate_clockwise().into())
                .collect(),
        }
    }
    fn rotate_counter_clockwise(&self) -> Self {
        Self {
            offsets: self
                .offsets
                .iter()
                .map(|sm| {
                    MapPoint::<X, Y>::from(*sm)
                        .rotate_counter_clockwise()
                        .into()
                })
                .collect(),
        }
    }
    fn flip_horizontal(&self) -> Self {
        Self {
            offsets: self
                .offsets
                .iter()
                .map(|sm| MapPoint::<X, Y>::from(*sm).flip_horizontal().into())
                .collect(),
        }
    }
    fn flip_vertical(&self) -> Self {
        Self {
            offsets: self
                .offsets
                .iter()
                .map(|sm| MapPoint::<X, Y>::from(*sm).flip_vertical().into())
                .collect(),
        }
    }
    fn flip_flop(&self) -> Self {
        Self {
            offsets: self
                .offsets
                .iter()
                .map(|sm| {
                    MapPoint::<X, Y>::from(*sm)
                        .flip_vertical()
                        .flip_horizontal()
                        .into()
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ImageTile {
    raw: MyMap2D<char, 10, 10>,
    value: u64,
    borders: [u16; 4],
    neighbors: [Option<usize>; 4],
}

impl From<&str> for ImageTile {
    fn from(value: &str) -> Self {
        let (value, raw) = value.split_once(":\n").unwrap();
        let value = value.strip_prefix("Tile ").unwrap();
        let value = value.parse().unwrap();
        let raw = MyMap2D::from(raw);
        // borders are read clockwise around tile, starting with top
        let borders = (0..10).fold([0; 4], |mut b, i| {
            let bits = [
                if *raw.get((i, 0).into()) == '#' { 1 } else { 0 },
                if *raw.get((9, i).into()) == '#' { 1 } else { 0 },
                if *raw.get((9 - i, 9).into()) == '#' {
                    1
                } else {
                    0
                },
                if *raw.get((0, 9 - i).into()) == '#' {
                    1
                } else {
                    0
                },
            ];
            b.iter_mut().zip(bits).for_each(|(border, bit)| {
                *border = (*border << 1) + bit;
            });
            b
        });
        ImageTile {
            raw,
            value,
            borders,
            neighbors: [None; 4],
        }
    }
}

fn invert_border(border: u16) -> u16 {
    border.reverse_bits() >> 6
}

impl ImageTile {
    fn set_neighbors(&mut self, border_pos: usize, index: usize) {
        self.neighbors[border_pos] = Some(index);
    }
    fn compare_borders(&self, others: Self) -> Option<(usize, usize)> {
        let mut border_fits: Vec<(usize, usize)> = Vec::new();
        for (i_s, i_b) in self.borders.iter().enumerate() {
            for (o_s, o_b) in others.borders.iter().enumerate() {
                if i_b == o_b {
                    border_fits.push((i_s, o_s));
                } else {
                    let inverted_border = invert_border(*o_b);
                    if *i_b == inverted_border {
                        border_fits.push((i_s, o_s));
                    }
                }
            }
        }
        match border_fits.len() {
            0 => None,
            1 => Some(border_fits[0]),
            _ => panic!("two tiles have too many common borders."),
        }
    }
    fn rotate_clockwise(&mut self) {
        self.raw = self.raw.rotate_clockwise();
        self.borders.rotate_right(1);
        self.neighbors.rotate_right(1);
    }
    fn flip_horizontal(&mut self) {
        self.raw = self.raw.flip_horizontal();
        /*for index in [1, 3] {
            self.borders[index] = invert_border(self.borders[index]);
        }*/
        self.neighbors.swap(0, 2);
        self.borders.swap(0, 2);
        for border in self.borders.iter_mut() {
            *border = invert_border(*border);
        }
    }
    fn flip_vertical(&mut self) {
        self.raw = self.raw.flip_vertical();
        /*for index in [0, 2] {
            self.borders[index] = invert_border(self.borders[index]);
        }*/
        self.neighbors.swap(1, 3);
        self.borders.swap(1, 3);
        for border in self.borders.iter_mut() {
            *border = invert_border(*border);
        }
    }
}

struct ChallengeInput<const X: usize> {
    tiles: Vec<ImageTile>,
    image: MyMap2D<char, X, X>,
}

impl<const X: usize> From<&str> for ChallengeInput<X> {
    fn from(value: &str) -> Self {
        ChallengeInput {
            tiles: value.split("\n\n").map(ImageTile::from).collect(),
            image: MyMap2D::default(),
        }
    }
}

impl<const X: usize> ChallengeInput<X> {
    fn solution_part_1_and_2(&mut self) -> (u64, usize) {
        let corner_indices = self.search_corners();
        let result_part_1 = corner_indices
            .iter()
            .map(|i| self.tiles[*i].value)
            .product();
        self.build_image_from_corner(corner_indices[0]);
        self.sea_monsters();
        (
            result_part_1,
            self.image.iter().filter(|(_, v)| **v == '#').count(),
        )
    }
    fn set_neighbors(&mut self) {
        for i_1 in 0..self.tiles.len() {
            for i_2 in i_1 + 1..self.tiles.len() {
                if let Some((b_1, b_2)) = self.tiles[i_1].compare_borders(self.tiles[i_2]) {
                    self.tiles[i_1].set_neighbors(b_1, i_2);
                    self.tiles[i_2].set_neighbors(b_2, i_1);
                }
            }
        }
    }
    fn search_corners(&mut self) -> Vec<usize> {
        self.set_neighbors();
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(i, t)| (t.neighbors.iter().filter_map(|n| *n).count() == 2).then_some(i))
            .collect()
    }
    fn build_image_from_corner(&mut self, corner_index: usize) {
        // we expect image to be quadratic
        // set first corner to top left -> rotate until neighbors are [None, Some(East), Some(South), None]
        while !matches!(
            self.tiles[corner_index].neighbors,
            [None, Some(_), Some(_), None]
        ) {
            self.tiles[corner_index].rotate_clockwise();
        }

        // order tiles, rotate and flip if required
        let mut tile_positions: Vec<usize> = Vec::new();
        let mut current_index = corner_index;
        for _row in 0..X / 8 {
            let row_start = current_index;
            tile_positions.push(current_index);
            while let Some(east) = self.tiles[current_index].neighbors[1] {
                // rotate east until current_index is west: [_, _, _, Some(current_index)]
                while self.tiles[east].neighbors[3] != Some(current_index) {
                    self.tiles[east].rotate_clockwise();
                }
                // check if horizontal flip is required
                // to be identical borders, one border must first be rotated
                if self.tiles[current_index].borders[1]
                    != invert_border(self.tiles[east].borders[3])
                {
                    self.tiles[east].flip_horizontal();
                    assert_eq!(
                        self.tiles[current_index].borders[1],
                        invert_border(self.tiles[east].borders[3])
                    );
                }
                current_index = east;
                tile_positions.push(current_index);
            }
            // get south of row start
            if let Some(south) = self.tiles[row_start].neighbors[2] {
                // rotate south until row_start is north: [Some(row_start), _, _, _]
                while self.tiles[south].neighbors[0] != Some(row_start) {
                    self.tiles[south].rotate_clockwise();
                }
                // check if vertical flip is required
                if self.tiles[row_start].borders[2] != invert_border(self.tiles[south].borders[0]) {
                    self.tiles[south].flip_vertical();
                    assert_eq!(
                        self.tiles[row_start].borders[2],
                        invert_border(self.tiles[south].borders[0])
                    );
                }
                current_index = south;
            }
        }
        // check all tiles are used
        assert_eq!(tile_positions.len(), self.tiles.len());
        for (index, pos_1) in tile_positions.iter().enumerate() {
            for pos_2 in tile_positions.iter().skip(index + 1) {
                assert_ne!(pos_1, pos_2);
            }
        }
        // build image from ordered tiles
        for y in 0..X {
            for x in 0..X {
                // current position of tile in image
                let tile_pos_x = x / 8;
                let tile_pos_y = y / 8;
                let tile_index = tile_pos_x + tile_pos_y * (X / 8);
                let tile_index = tile_positions[tile_index];
                // current index in tile (add +1 since we only take the inner 8 x 8 sub image)
                let tile_x = 1 + x % 8;
                let tile_y = 1 + y % 8;
                self.image.set(
                    (x, y).into(),
                    *self.tiles[tile_index].raw.get((tile_x, tile_y).into()),
                );
            }
        }
    }
    fn sea_monsters(&mut self) {
        let sea_monster = SeaMonster::<20, 3>::new();
        let mut sm_offsets: Vec<(usize, usize)> = Vec::new();
        'outer: for rot in 0..3 {
            for flip in 0..4 {
                let offsets = match (rot, flip) {
                    (0, 0) => &sea_monster.offsets,
                    (0, 1) => &sea_monster.flip_horizontal().offsets,
                    (0, 2) => &sea_monster.flip_vertical().offsets,
                    (0, _) => &sea_monster.flip_flop().offsets,
                    (1, 0) => &sea_monster.rotate_clockwise().offsets,
                    (1, 1) => &sea_monster.rotate_clockwise().flip_horizontal().offsets,
                    (1, 2) => &sea_monster.rotate_clockwise().flip_vertical().offsets,
                    (1, _) => &sea_monster.rotate_clockwise().flip_flop().offsets,
                    (2, 0) => &sea_monster.rotate_counter_clockwise().offsets,
                    (2, 1) => {
                        &sea_monster
                            .rotate_counter_clockwise()
                            .flip_horizontal()
                            .offsets
                    }
                    (2, 2) => {
                        &sea_monster
                            .rotate_counter_clockwise()
                            .flip_vertical()
                            .offsets
                    }
                    (_, _) => &sea_monster.rotate_counter_clockwise().flip_flop().offsets,
                };
                for y in 0..X {
                    for x in 0..X {
                        let anchor = MapPoint::<X, X>::new(x, y);
                        if offsets
                            .iter()
                            .all(|offset| match anchor.offset_pp(*offset) {
                                Some(sm_pos) => *self.image.get(sm_pos) == '#',
                                None => false,
                            })
                        {
                            // found a sea monster
                            sm_offsets = offsets.clone();
                            break 'outer;
                        }
                    }
                }
            }
        }
        if sm_offsets.is_empty() {
            panic!("could not find any sea monster.");
        }
        // mask sea monster in image
        let mut anchors: Vec<MapPoint<X, X>> = Vec::new();
        for y in 0..X {
            for x in 0..X {
                let anchor = MapPoint::<X, X>::new(x, y);
                if sm_offsets
                    .iter()
                    .all(|offset| match anchor.offset_pp(*offset) {
                        Some(sm_pos) => *self.image.get(sm_pos) == '#',
                        None => false,
                    })
                {
                    // found a sea monster
                    anchors.push(anchor);
                }
            }
        }
        for anchor in anchors {
            sm_offsets
                .iter()
                .filter_map(|offset| anchor.offset_pp(*offset))
                .for_each(|sm_pos| {
                    self.image.set(sm_pos, 'O');
                });
        }
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2020/day_20.txt");
    let mut challenge = ChallengeInput::<96>::from(input);
    dbg!(challenge.tiles.len());

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_20 part 1: {result_part1}");
    assert_eq!(result_part1, 64_802_175_715_999);

    println!("result day_20 part 2: {result_part2}");
    assert_eq!(result_part2, 2_146);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_20() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2020/day_20_example.txt");
        let mut example = ChallengeInput::<24>::from(input);

        let (result_part1, result_part2) = example.solution_part_1_and_2();
        println!("result day_20 part 1: {result_part1}");
        assert_eq!(result_part1, 20_8990_4808_3289);

        println!("result day_20 part 2: {result_part2}");
        assert_eq!(result_part2, 273);

        Ok(())
    }
}
