//!day_10.rs

use anyhow::{Result, anyhow};
use my_lib::my_compass::Compass;
use my_lib::my_map_point::MapPoint;
use my_lib::my_map_two_dim::MyMap2D;

// values taken from ../../../../aoc_input/aoc-2023/day_10.txt
// number of chars in one line
const X: usize = 140;
// number of lines
const Y: usize = 140;

#[derive(Copy, Clone, PartialEq, Default)]
enum PipeSegment {
    Pipe,
    LeftSide,
    RightSide,
    #[default]
    None,
}

impl PipeSegment {
    fn is_side_segment(&self) -> bool {
        matches!(self, PipeSegment::LeftSide | PipeSegment::RightSide)
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Pipe {
    layout: MyMap2D<PipeSegment, 3, 3>,
}

impl Default for Pipe {
    fn default() -> Self {
        let mut pipe = Pipe {
            layout: MyMap2D::default(),
        };
        pipe.layout.set(Pipe::center(), PipeSegment::Pipe);
        pipe
    }
}

impl From<char> for Pipe {
    // pipe types
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile --> is NOT PIPE!
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    fn from(value: char) -> Self {
        let mut pipe = Pipe::default();
        match value {
            '|' => {
                pipe.set(Compass::N, PipeSegment::Pipe).unwrap();
                pipe.set(Compass::S, PipeSegment::Pipe).unwrap();
            }
            '-' => {
                pipe.set(Compass::E, PipeSegment::Pipe).unwrap();
                pipe.set(Compass::W, PipeSegment::Pipe).unwrap();
            }
            'L' => {
                pipe.set(Compass::N, PipeSegment::Pipe).unwrap();
                pipe.set(Compass::E, PipeSegment::Pipe).unwrap();
            }
            'J' => {
                pipe.set(Compass::N, PipeSegment::Pipe).unwrap();
                pipe.set(Compass::W, PipeSegment::Pipe).unwrap();
            }
            '7' => {
                pipe.set(Compass::S, PipeSegment::Pipe).unwrap();
                pipe.set(Compass::W, PipeSegment::Pipe).unwrap();
            }
            'F' => {
                pipe.set(Compass::S, PipeSegment::Pipe).unwrap();
                pipe.set(Compass::E, PipeSegment::Pipe).unwrap();
            }
            'S' => (),
            _ => panic!("bad pipe char"),
        }
        pipe
    }
}

impl Pipe {
    fn center() -> MapPoint<3, 3> {
        MapPoint::<3, 3>::new(1, 1)
    }
    fn from_neighbors(neighbors: &[(Option<char>, Compass)]) -> Result<Pipe> {
        // if neighbors.len() is > 4,
        if neighbors.len() > 4 {
            return Err(anyhow!("neighbors.len() > 4"));
        }
        let mut pipe = Pipe::default();
        let mut found_gates: usize = 0;
        for (op, o) in neighbors.iter().map(|(c, o)| (c.map(Pipe::from), o)) {
            if let Some(neighbor_pipe) = op
                && neighbor_pipe.has_gate(o.flip())?
            {
                if found_gates == 2 {
                    return Err(anyhow!("pipe only supports two gates"));
                }
                pipe.set(*o, PipeSegment::Pipe)?;
                found_gates += 1;
            }
        }
        if found_gates < 2 {
            Err(anyhow!("tile is not pipe"))
        } else {
            Ok(pipe)
        }
    }
    fn get_segment(&self, orientation: Compass) -> Result<PipeSegment> {
        if orientation.is_center() {
            return Err(anyhow!("only cardinal and ordinal orientation allowed"));
        }
        Ok(*self
            .layout
            .get(Pipe::center().neighbor(orientation).unwrap()))
    }
    fn get_gates(&self) -> Option<(Compass, Compass)> {
        let mut iter_gates = self
            .layout
            .iter_neighbors(Pipe::center())
            .filter(|(.., ps)| **ps == PipeSegment::Pipe)
            .map(|(_, o, _)| o);
        let gate_1 = iter_gates.next()?;
        let gate_2 = iter_gates.next().unwrap();
        Some((gate_1, gate_2))
    }
    fn set(&mut self, orientation: Compass, segment: PipeSegment) -> Result<PipeSegment> {
        if !orientation.is_cardinal() {
            return Err(anyhow!("only cardinal orientation allowed"));
        }
        Ok(*self
            .layout
            .set(Pipe::center().neighbor(orientation).unwrap(), segment))
    }
    fn has_gate(&self, orientation: Compass) -> Result<bool> {
        if !orientation.is_cardinal() {
            return Err(anyhow!("only cardinal orientation allowed"));
        }
        Ok(*self
            .layout
            .get(Pipe::center().neighbor(orientation).unwrap())
            == PipeSegment::Pipe)
    }
    fn init_pipe_sides(&mut self) -> Result<Compass> {
        let (exit_gate, _) = self
            .get_gates()
            .ok_or(anyhow!("init_pipe_sides: tile is not a pipe"))?;
        let current_side = PipeSegment::LeftSide;
        self.set_pipe_side(exit_gate, current_side);
        Ok(exit_gate)
    }
    fn extend_pipe_sides(&mut self, flow_direction: Compass, previous: Pipe) -> Result<()> {
        // flow_direction points toward tile from previous; flip() to gate initial entry gate
        let entry_gate = flow_direction.flip();
        // safety check
        previous.has_gate(flow_direction)?;
        self.has_gate(entry_gate)?;
        let (current_side, initial_orientation) = match flow_direction {
            Compass::N => (previous.get_segment(Compass::NE)?, Compass::SE),
            Compass::E => (previous.get_segment(Compass::SE)?, Compass::SW),
            Compass::S => (previous.get_segment(Compass::SW)?, Compass::NW),
            Compass::W => (previous.get_segment(Compass::NW)?, Compass::NE),
            _ => {
                return Err(anyhow!(
                    "this internal error is not possible because of safety checks"
                ));
            }
        };
        self.set_pipe_side(initial_orientation, current_side);
        Ok(())
    }
    fn set_pipe_side(&mut self, initial_orientation: Compass, mut current_side: PipeSegment) {
        for (segment, _) in Pipe::center().iter_neighbors(initial_orientation, true, false, true) {
            if *self.layout.get(segment) == PipeSegment::Pipe {
                current_side = match current_side {
                    PipeSegment::LeftSide => PipeSegment::RightSide,
                    _ => PipeSegment::LeftSide,
                };
            } else {
                self.layout.set(segment, current_side);
            }
        }
    }
    fn iter_pipe_segments(&self) -> impl Iterator<Item = (&PipeSegment, Compass)> {
        self.layout
            .iter_neighbors_with_corners(Pipe::center())
            .map(|(_, o, ps)| (ps, o))
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Unknown(char),
    Pipe(Pipe),
    LeftOfPipe,
    RightOfPipe,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Unknown('.')
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Tile::Unknown(value)
    }
}

impl Tile {
    fn get_unknown_tile_char(&self) -> Option<char> {
        match self {
            Tile::Unknown(c) => Some(*c),
            _ => None,
        }
    }
    fn set_start_pipe(&mut self, pipe: Pipe) -> Result<Pipe> {
        match self {
            Tile::Unknown(c) => {
                if *c == 'S' {
                    *self = Tile::Pipe(pipe);
                    Ok(pipe)
                } else {
                    Err(anyhow!("Tile is not start tile"))
                }
            }
            Tile::Pipe(_) => Err(anyhow!("Tile is already pipe")),
            _ => Err(anyhow!("unable to convert to pipe.")),
        }
    }
    fn change_to_pipe(&mut self) -> Result<Pipe> {
        match self {
            Tile::Unknown(c) => {
                let pipe = Pipe::from(*c);
                *self = Tile::Pipe(pipe);
                Ok(pipe)
            }
            Tile::Pipe(pipe) => Ok(*pipe),
            _ => Err(anyhow!("unable to convert to pipe.")),
        }
    }
    fn get_pipe(&self) -> Option<&Pipe> {
        match self {
            Tile::Pipe(pipe) => Some(pipe),
            _ => None,
        }
    }
    fn get_pipe_mut(&mut self) -> Option<&mut Pipe> {
        match self {
            Tile::Pipe(pipe) => Some(pipe),
            _ => None,
        }
    }
    fn change_to_left_of_pipe(&mut self) {
        // do not switch if pipe
        match self {
            Tile::Pipe(_) => (),
            _ => *self = Tile::LeftOfPipe,
        }
    }
    fn change_to_right_of_pipe(&mut self) {
        // do not switch if pipe
        match self {
            Tile::Pipe(_) => (),
            _ => *self = Tile::RightOfPipe,
        }
    }
    fn is_side_tile(&self) -> Option<Self> {
        match self {
            Tile::LeftOfPipe | Tile::RightOfPipe => Some(*self),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Default)]
struct TileMap<const X: usize, const Y: usize> {
    map: MyMap2D<Tile, X, Y>,
    start_tile: MapPoint<X, Y>,
    number_of_pipe_tiles: usize,
    outside: Option<Tile>,
}

impl<const X: usize, const Y: usize> From<&str> for TileMap<X, Y> {
    fn from(value: &str) -> Self {
        let mut map = TileMap::<X, Y>::default();
        for (y, line) in value.trim().lines().enumerate() {
            for (x, pipe_char) in line.trim().chars().enumerate() {
                *map.map.get_mut(MapPoint::<X, Y>::new(x, y)) = Tile::from(pipe_char);
                if pipe_char == 'S' {
                    map.start_tile = MapPoint::<X, Y>::new(x, y);
                }
            }
        }
        map
    }
}

impl<const X: usize, const Y: usize> TileMap<X, Y> {
    fn get_number_of_pipe_tiles(&self) -> usize {
        self.number_of_pipe_tiles
    }
    fn set_start_pipe(&mut self) -> Result<(MapPoint<X, Y>, Compass)> {
        let neighbor_tiles: Vec<(Option<char>, Compass)> = self
            .map
            .iter_neighbors(self.start_tile)
            .map(|(_, o, t)| (t.get_unknown_tile_char(), o))
            .collect();
        let start_pipe = Pipe::from_neighbors(&neighbor_tiles[..])?;
        self.map
            .get_mut(self.start_tile)
            .set_start_pipe(start_pipe)?;
        // add start pipe to number of pipes
        self.number_of_pipe_tiles += 1;
        // initiate flow direction and next_tile
        let (flow_direction, _) = start_pipe
            .get_gates()
            .ok_or(anyhow!("internal error set_start_pipe: get_gates"))?;
        let next_tile = self.flow_to_next_tile(self.start_tile, flow_direction)?;
        Ok((next_tile, flow_direction))
    }
    fn is_start_pipe(&self, current_tile: MapPoint<X, Y>) -> bool {
        self.start_tile == current_tile
    }
    fn flow_to_next_tile(
        &self,
        current_tile: MapPoint<X, Y>,
        flow_direction: Compass,
    ) -> Result<MapPoint<X, Y>> {
        // flow_direction points from current tile toward next tile with pipe
        current_tile
            .neighbor(flow_direction)
            .ok_or(anyhow!("flow direction points outside of map"))
    }
    fn tile_to_pipe(
        &mut self,
        current_tile: MapPoint<X, Y>,
        flow_direction: Compass,
    ) -> Result<Compass> {
        // unwrap at the is ok, since we know, that without an error in change_to_pipe(), get_gates will return pipe gates
        let (gate_1, gate_2) = self
            .map
            .get_mut(current_tile)
            .change_to_pipe()?
            .get_gates()
            .unwrap();
        //increment pipe Counter
        self.number_of_pipe_tiles += 1;
        // flow_direction points toward current_tile. By flipping it it points to entry gate of pipe
        let entry_gate = flow_direction.flip();
        if entry_gate != gate_1 && entry_gate != gate_2 {
            Err(anyhow!("Stuck at non matching pipe at {}", current_tile))
        } else if entry_gate == gate_1 {
            Ok(gate_2)
        } else {
            Ok(gate_1)
        }
    }
    fn check_pipe_gate(
        &self,
        tile_to_check: MapPoint<X, Y>,
        flow_direction: Compass,
    ) -> Result<bool> {
        // flow_direction points toward tile_to_check
        self.map
            .get(tile_to_check)
            .get_pipe()
            .ok_or(anyhow!("tile is not a pipe"))?
            .has_gate(flow_direction.flip())
    }
    fn init_pipe_sides(&mut self) -> Result<(MapPoint<X, Y>, Compass, Pipe)> {
        let start_pipe = self
            .map
            .get_mut(self.start_tile)
            .get_pipe_mut()
            .ok_or(anyhow!("start tile is not a pipe"))?;
        let flow_direction = start_pipe.init_pipe_sides()?;
        let start_pipe = *start_pipe;
        let next_tile = self.flow_to_next_tile(self.start_tile, flow_direction)?;
        self.set_pipe_side_tiles(next_tile)?;
        Ok((next_tile, flow_direction, start_pipe))
    }
    fn extend_pipe_sides(
        &mut self,
        current_tile: MapPoint<X, Y>,
        flow_direction: Compass,
        previous_pipe: Pipe,
    ) -> Result<(Compass, Pipe)> {
        self.map
            .get_mut(current_tile)
            .get_pipe_mut()
            .ok_or(anyhow!("tile is not a pipe"))?
            .extend_pipe_sides(flow_direction, previous_pipe)?;
        self.set_pipe_side_tiles(current_tile)?;
        let pipe = *self.map.get(current_tile).get_pipe().unwrap();
        let (gate_1, gate_2) = pipe.get_gates().unwrap();
        let entry_gate = flow_direction.flip();
        if entry_gate != gate_1 && entry_gate != gate_2 {
            Err(anyhow!("Stuck at non matching pipe at {}", current_tile))
        } else if entry_gate == gate_1 {
            Ok((gate_2, pipe))
        } else {
            Ok((gate_1, pipe))
        }
    }
    fn set_pipe_side_tiles(&mut self, current_tile: MapPoint<X, Y>) -> Result<()> {
        let pipe = *self
            .map
            .get(current_tile)
            .get_pipe()
            .ok_or(anyhow!("tile is not a pipe"))?;
        for (segment, orientation) in pipe
            .iter_pipe_segments()
            .filter(|(ps, _)| ps.is_side_segment())
        {
            // at sidelines of map neighbor could be None
            if let Some(neighbor) = current_tile.neighbor(orientation) {
                match segment {
                    PipeSegment::LeftSide => self.map.get_mut(neighbor).change_to_left_of_pipe(),
                    PipeSegment::RightSide => self.map.get_mut(neighbor).change_to_right_of_pipe(),
                    _ => {
                        return Err(anyhow!(
                            "internal error, cannot happen because of filter in iter"
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    fn extend_side_tiles(&mut self) {
        let sides_to_extend = [Tile::LeftOfPipe, Tile::RightOfPipe];
        for side_to_extend in sides_to_extend.into_iter() {
            loop {
                let tile = match self
                    .map
                    .iter()
                    .find(|(p, t)| {
                        **t == side_to_extend
                            && self
                                .map
                                .iter_neighbors(*p)
                                .any(|(.., nt)| nt.get_unknown_tile_char().is_some())
                    })
                    .map(|(p, _)| p)
                {
                    Some(point) => point,
                    None => break,
                };

                let mut tiles_to_extend_to: Vec<MapPoint<X, Y>> = vec![tile];
                let mut index = 0;
                while index < tiles_to_extend_to.len() {
                    let check_tile = tiles_to_extend_to[index];
                    *self.map.get_mut(check_tile) = side_to_extend;
                    let neighbors_to_extend: Vec<MapPoint<X, Y>> = self
                        .map
                        .iter_neighbors(check_tile)
                        .filter(|(np, _, nt)| {
                            nt.get_unknown_tile_char().is_some()
                                && !tiles_to_extend_to.iter().any(|p| p == np)
                        })
                        .map(|(p, ..)| p)
                        .collect();
                    tiles_to_extend_to.extend_from_slice(&neighbors_to_extend[..]);
                    index += 1;
                }
            }
        }
    }
    fn identify_outside_count_inside(&mut self) -> Result<usize> {
        // this works only, if there is at least one side tile of pipe, which is at side of map
        self.outside = self
            .map
            .iter()
            .filter(|(p, ..)| p.map_position() != Compass::Center)
            .find(|(.., t)| t.is_side_tile().is_some())
            .map(|(.., t)| t.is_side_tile().unwrap());
        let inside = match self.outside {
            Some(side) => match side {
                Tile::LeftOfPipe => Tile::RightOfPipe,
                Tile::RightOfPipe => Tile::LeftOfPipe,
                _ => return Err(anyhow!("internal error")),
            },
            None => return Err(anyhow!("outside not found")),
        };
        Ok(self
            .map
            .iter()
            .filter(|(.., t)| t.is_side_tile().is_some_and(|t| t == inside))
            .count())
    }
}

pub fn day_10() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_10.txt");

    // part 1: build pipe map and count pipe tiles
    let mut tile_map = TileMap::<X, Y>::from(input);
    // flow_direction points toward next tile with pipe
    let (mut current_tile, mut flow_direction) = tile_map.set_start_pipe()?;
    // safety_counter to prevent infinite loop in case of error
    let mut safety_counter = X * Y;
    // let it flow and build the pipe
    while !tile_map.is_start_pipe(current_tile) && safety_counter > 0 {
        safety_counter -= 1;
        flow_direction = tile_map.tile_to_pipe(current_tile, flow_direction)?;
        current_tile = tile_map.flow_to_next_tile(current_tile, flow_direction)?;
    }
    // now current_tile is starting_tile and flow_direction points toward unused gate of starting_tile
    // check if starting_tile has this gate
    if !tile_map.check_pipe_gate(current_tile, flow_direction)? {
        return Err(anyhow!("pipe is broken"));
    }

    let farthest_distance_from_start_tile_trough_pipe =
        tile_map.get_number_of_pipe_tiles() / 2 + tile_map.get_number_of_pipe_tiles() % 2;
    println!(
        "result day 10 part 1: {}",
        farthest_distance_from_start_tile_trough_pipe
    );
    assert_eq!(farthest_distance_from_start_tile_trough_pipe, 6_697);

    // part 2: identify left and right side tiles of pipe and check, which one is outside
    // count outside tiles as result
    // first set all tiles, which are not pipes and are neighboring to a pipe tile to LeftSide or RightSide
    // by flowing again through pipe, but this time set pipe sides
    let (mut current_tile, mut flow_direction, mut previous_pipe) = tile_map.init_pipe_sides()?;
    while !tile_map.is_start_pipe(current_tile) {
        let extend_result =
            tile_map.extend_pipe_sides(current_tile, flow_direction, previous_pipe)?;
        flow_direction = extend_result.0;
        previous_pipe = extend_result.1;
        current_tile = tile_map.flow_to_next_tile(current_tile, flow_direction)?;
    }

    // now extend pipe sides to all tiles, which are not directly connected to a pipe tile
    tile_map.extend_side_tiles();
    // identify outside and count inside
    let number_of_inside_tiles = tile_map.identify_outside_count_inside()?;
    println!("result day 10 part 2: {}", number_of_inside_tiles);
    assert_eq!(number_of_inside_tiles, 423);

    Ok(())
}
