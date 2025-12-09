//!day_09.rs

use anyhow::Result;
use my_lib::my_geometry::{my_line::LineSegment, my_point::Point, my_rectangle::Rectangle};
use std::collections::HashSet;

struct ChallengeInput {
    tiles: Vec<Point>,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            tiles: value
                .lines()
                .filter_map(|l| l.split_once(','))
                .map(|(x, y)| Point::new(x.parse().unwrap(), y.parse().unwrap()))
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1(&self) -> i64 {
        let mut max_rectangle_size = i64::MIN;
        for (i, tile_a) in self.tiles.iter().enumerate() {
            for tile_b in self.tiles.iter().skip(i + 1) {
                max_rectangle_size =
                    max_rectangle_size.max(Rectangle::from((*tile_a, *tile_b)).surface_inclusive());
            }
        }
        max_rectangle_size
    }
    /// Solution:
    /// 1.) collect all tile segments
    /// 2.) create rectangle from pair if tiles
    /// 3.) check for red tiles on sides of rectangle and intersections of segments with rectangle: checkpoint
    /// 4.) select neighboring tile of checkpoint, which is although on side of rectangle, but not on any segment
    /// (if outside corner, there is no neighboring tile to check)
    /// We check for every neighbor, if he is inside or outside of tile space by counting intersections with segments:
    /// 5.) create a segment from neighbor tile to the right (i64::MAX) and count intersections
    /// with segments: special rules:
    /// 5.1) If intersection is endpoint of corner, ignore it.
    /// 5.2) If intersection is endpoint of two segments in the same direction (both vertical
    /// or horizontal), count this as one intersection.
    /// 6.) Check count. If
    /// 6.1) odd intersections: point is inside -> go to 3.) and check next tile
    /// 6.2) even intersections, but not zero: point is outside -> discard rectangle, go to 2.) for next rectangle
    /// 6.3) if zero, do the same for a vertical segment starting from tile, but this time 0 counts as even.
    /// 7.) If all checks are ok (meaning odd), calc area of rectangle, check for MaxArea and go to step 2.) for next rectangle
    /// 8.) if no more rectangles, return MaxArea
    fn solution_part_2(&self) -> i64 {
        // collect segments
        let mut segments: Vec<LineSegment> = Vec::with_capacity(self.tiles.len());
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;
        for (i, tile_a) in self.tiles.iter().enumerate() {
            max_x = max_x.max(tile_a.x);
            max_y = max_y.max(tile_a.y);
            let i = (i + 1) % self.tiles.len();
            let tile_b = self.tiles[i];
            segments.push(LineSegment::new(*tile_a, tile_b));
        }
        max_x *= 10;
        max_y *= 10;
        // collect same direction endpoints
        let same_direction_endpoints: HashSet<Point> = self
            .tiles
            .iter()
            .filter(|t| {
                let tile_segments: Vec<LineSegment> =
                    segments.iter().filter(|ts| ts == t).copied().collect();
                assert_eq!(tile_segments.len(), 2);
                tile_segments[0].is_parallel(&tile_segments[1])
            })
            .copied()
            .collect();
        // check rectangles
        let mut max_rectangle_size = i64::MIN;
        for (i, tile_a) in self.tiles.iter().enumerate() {
            'rectangle_loop: for tile_b in self.tiles.iter().skip(i + 1) {
                let rectangle = Rectangle::from((*tile_a, *tile_b));
                for side in rectangle.sides() {
                    // LineSegment == Point is true, if Point is on LineSegment
                    for tile_on_side in self
                        .tiles
                        .iter()
                        .filter(|p| side == **p)
                        .copied()
                        // intersections of segments with tiles. This is important, since line segments of tiles may be parallel
                        // and direct neighbors, therefore creating a surface of valid tiles. We need to find neighbors of
                        // valid tiles on side of rectangle and check, if they are inside or outside of valid tile area
                        .chain(segments.iter().filter_map(|s| side.segment_intersection(s)))
                    {
                        for neighbor in [(1, 0), (-1, 0), (0, 1), (0, -1)]
                            .into_iter()
                            .map(|d| tile_on_side.add(d))
                            .filter(|n| side == *n && segments.iter().all(|s| s != n))
                        {
                            let to_the_right =
                                LineSegment::new(neighbor, (max_x, neighbor.y).into());
                            let mut same_direction_count = 0;
                            let mut intersection_count = segments
                                .iter()
                                .filter(|s| {
                                    if let Some(is) = s.segment_intersection(&to_the_right) {
                                        if same_direction_endpoints.contains(&is) {
                                            same_direction_count += 1;
                                            true
                                        } else {
                                            !s.end_points().contains(&is)
                                        }
                                    } else {
                                        false
                                    }
                                })
                                .count();
                            // remove same direction double count
                            intersection_count -= same_direction_count / 2;
                            if intersection_count.is_multiple_of(2) {
                                if intersection_count > 0 {
                                    continue 'rectangle_loop;
                                }
                                let to_the_top =
                                    LineSegment::new(neighbor, (neighbor.x, max_y).into());
                                let mut same_direction_count = 0;
                                let intersection_count = segments
                                    .iter()
                                    .filter(|s| {
                                        if let Some(is) = s.segment_intersection(&to_the_top) {
                                            if same_direction_endpoints.contains(&is) {
                                                same_direction_count += 1;
                                                true
                                            } else {
                                                !s.end_points().contains(&is)
                                            }
                                        } else {
                                            false
                                        }
                                    })
                                    .count();
                                if intersection_count.is_multiple_of(2) {
                                    continue 'rectangle_loop;
                                }
                            }
                        }
                    }
                }
                max_rectangle_size = max_rectangle_size.max(rectangle.surface_inclusive());
            }
        }

        max_rectangle_size
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2025/day_09.txt");
    let challenge = ChallengeInput::from(input);

    let result_part1 = challenge.solution_part_1();
    println!("result day_09 part 1: {result_part1}");
    assert_eq!(result_part1, 4_759_531_084);

    let result_part2 = challenge.solution_part_2();
    println!("result day_09 part 2: {result_part2}");
    assert_eq!(result_part2, 1_539_238_860);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_day_09() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2025/day_09_example.txt");
        let example = ChallengeInput::from(input);

        let result_part1 = example.solution_part_1();
        println!("result day_09 part 1: {result_part1}");
        assert_eq!(result_part1, 50);

        let result_part2 = example.solution_part_2();
        println!("result day_09 part 2: {result_part2}");
        assert_eq!(result_part2, 24);

        Ok(())
    }
}
