//!day_17.rs

use anyhow::Result;
#[cfg(feature = "long-run-time")]
use my_lib::my_compass::Compass;
#[cfg(feature = "long-run-time")]
use my_lib::my_map_point::MapPoint;

#[cfg(feature = "long-run-time")]
use anyhow::anyhow;
#[cfg(feature = "long-run-time")]
use my_lib::my_map_two_dim::MyMap2D;

#[cfg(feature = "long-run-time")]
use std::cmp::Ordering;
#[cfg(feature = "long-run-time")]
use std::collections::BinaryHeap;

// values taken from ../../../../aoc_input/aoc-2023/day_17.txt
#[cfg(feature = "long-run-time")]
const X: usize = 141;
#[cfg(feature = "long-run-time")]
const Y: usize = 141;

#[cfg(feature = "long-run-time")]
trait PathNode<const X: usize, const Y: usize>: Default + PartialEq + Eq + Copy + Clone {
    fn step_forward(&self) -> Option<Self>;
    fn step_left(&self) -> Option<Self>;
    fn step_right(&self) -> Option<Self>;
    fn get_city_block(&self) -> MapPoint<X, Y>;
}

#[cfg(feature = "long-run-time")]
#[derive(Default, PartialEq, Eq, Copy, Clone)]
struct NormalCrucible<const X: usize, const Y: usize> {
    city_block: MapPoint<X, Y>,
    direction: Compass,
    n_steps: u8,
}

#[cfg(feature = "long-run-time")]
impl<const X: usize, const Y: usize> PathNode<X, Y> for NormalCrucible<X, Y> {
    fn step_forward(&self) -> Option<Self> {
        if self.direction.is_center() || self.n_steps == 3 {
            return None;
        }
        if let Some(next_city_block) = self.city_block.neighbor(self.direction) {
            return Some(Self {
                city_block: next_city_block,
                direction: self.direction,
                n_steps: self.n_steps + 1,
            });
        }
        None
    }
    fn step_left(&self) -> Option<Self> {
        if self.direction.is_center() {
            // start of search: left is Compass::E
            return Some(Self {
                city_block: self.city_block.neighbor(Compass::E).unwrap(),
                direction: Compass::E,
                n_steps: 1,
            });
        }
        let left = self.direction.counterclockwise().counterclockwise();
        if let Some(next_city_block) = self.city_block.neighbor(left) {
            return Some(Self {
                city_block: next_city_block,
                direction: left,
                n_steps: 1,
            });
        }
        None
    }
    fn step_right(&self) -> Option<Self> {
        if self.direction.is_center() {
            // start of search: right is Compass::S
            return Some(Self {
                city_block: self.city_block.neighbor(Compass::S).unwrap(),
                direction: Compass::S,
                n_steps: 1,
            });
        }
        let right = self.direction.clockwise().clockwise();
        if let Some(next_city_block) = self.city_block.neighbor(right) {
            return Some(Self {
                city_block: next_city_block,
                direction: right,
                n_steps: 1,
            });
        }
        None
    }
    fn get_city_block(&self) -> MapPoint<X, Y> {
        self.city_block
    }
}

#[cfg(feature = "long-run-time")]
#[derive(Default, PartialEq, Eq, Copy, Clone)]
struct UltraCrucible<const X: usize, const Y: usize> {
    city_block: MapPoint<X, Y>,
    direction: Compass,
    n_steps: u8,
}

#[cfg(feature = "long-run-time")]
impl<const X: usize, const Y: usize> PathNode<X, Y> for UltraCrucible<X, Y> {
    fn step_forward(&self) -> Option<Self> {
        if self.direction.is_center() || self.n_steps == 10 {
            return None;
        }
        if let Some(next_city_block) = self.city_block.neighbor(self.direction) {
            return Some(Self {
                city_block: next_city_block,
                direction: self.direction,
                n_steps: self.n_steps + 1,
            });
        }
        None
    }
    fn step_left(&self) -> Option<Self> {
        if self.direction.is_center() {
            // start of search: left is Compass::E
            return Some(Self {
                city_block: self.city_block.neighbor(Compass::E).unwrap(),
                direction: Compass::E,
                n_steps: 1,
            });
        }
        if self.n_steps < 4 {
            return None;
        }
        let left = self.direction.counterclockwise().counterclockwise();
        if let Some(next_city_block) = self.city_block.neighbor(left) {
            return Some(Self {
                city_block: next_city_block,
                direction: left,
                n_steps: 1,
            });
        }
        None
    }
    fn step_right(&self) -> Option<Self> {
        if self.direction.is_center() {
            // start of search: right is Compass::S
            return Some(Self {
                city_block: self.city_block.neighbor(Compass::S).unwrap(),
                direction: Compass::S,
                n_steps: 1,
            });
        }
        if self.n_steps < 4 {
            return None;
        }
        let right = self.direction.clockwise().clockwise();
        if let Some(next_city_block) = self.city_block.neighbor(right) {
            return Some(Self {
                city_block: next_city_block,
                direction: right,
                n_steps: 1,
            });
        }
        None
    }
    fn get_city_block(&self) -> MapPoint<X, Y> {
        self.city_block
    }
}

#[cfg(feature = "long-run-time")]
#[derive(Default, Eq, Copy, Clone)]
struct HeatPathNode<N: PathNode<X, Y>, const X: usize, const Y: usize> {
    accumulated_heat_loss: u64,
    path_node: N,
}

#[cfg(feature = "long-run-time")]
impl<N: PathNode<X, Y>, const X: usize, const Y: usize> PartialOrd for HeatPathNode<N, X, Y> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "long-run-time")]
impl<N: PathNode<X, Y>, const X: usize, const Y: usize> Ord for HeatPathNode<N, X, Y> {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse order to use std::collections::BinaryHeap as Min-heap by switching position of self and other
        // see https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#min-heap
        other.accumulated_heat_loss.cmp(&self.accumulated_heat_loss)
    }
}

#[cfg(feature = "long-run-time")]
impl<N: PathNode<X, Y>, const X: usize, const Y: usize> PartialEq for HeatPathNode<N, X, Y> {
    fn eq(&self, other: &Self) -> bool {
        self.accumulated_heat_loss == other.accumulated_heat_loss
    }
}

#[cfg(feature = "long-run-time")]
impl<N: PathNode<X, Y>, const X: usize, const Y: usize> HeatPathNode<N, X, Y> {
    fn step_forward(&self, map: &MyMap2D<u64, X, Y>) -> Option<Self> {
        if let Some(next_patch_node) = self.path_node.step_forward() {
            return Some(Self {
                accumulated_heat_loss: self.accumulated_heat_loss
                    + *map.get(next_patch_node.get_city_block()),
                path_node: next_patch_node,
            });
        }
        None
    }
    fn step_left(&self, map: &MyMap2D<u64, X, Y>) -> Option<Self> {
        if let Some(next_patch_node) = self.path_node.step_left() {
            return Some(Self {
                accumulated_heat_loss: self.accumulated_heat_loss
                    + *map.get(next_patch_node.get_city_block()),
                path_node: next_patch_node,
            });
        }
        None
    }
    fn step_right(&self, map: &MyMap2D<u64, X, Y>) -> Option<Self> {
        if let Some(next_patch_node) = self.path_node.step_right() {
            return Some(Self {
                accumulated_heat_loss: self.accumulated_heat_loss
                    + *map.get(next_patch_node.get_city_block()),
                path_node: next_patch_node,
            });
        }
        None
    }
}

#[cfg(feature = "long-run-time")]
struct CityMap<N: PathNode<X, Y>, const X: usize, const Y: usize> {
    map: MyMap2D<u64, X, Y>,
    seen_cache: Vec<N>,
    b_heap: BinaryHeap<HeatPathNode<N, X, Y>>,
}

#[cfg(feature = "long-run-time")]
impl<N: PathNode<X, Y>, const X: usize, const Y: usize> CityMap<N, X, Y> {
    fn new(input: &str) -> Self {
        let mut map: MyMap2D<u64, X, Y> = MyMap2D::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.set(
                    (x, y).into(),
                    c.to_digit(10).expect("bad city block char") as u64,
                );
            }
        }
        let mut city_map = CityMap {
            map,
            seen_cache: Vec::with_capacity(X * Y),
            b_heap: BinaryHeap::with_capacity(X),
        };
        // lava pool is at top-left city block, which is default value for HeatPathNode
        city_map.b_heap.push(HeatPathNode::<N, X, Y>::default());
        city_map
    }
    fn get_minimum_heat_loss(&mut self) -> Result<u64> {
        while !self.b_heap.is_empty() {
            // unwrap is safe, since we check for empty b_heap
            let current_node = self.b_heap.pop().unwrap();
            if current_node.path_node.get_city_block() == MapPoint::<X, Y>::new(X - 1, Y - 1) {
                return Ok(current_node.accumulated_heat_loss);
            }

            // check if we already run in this path
            if self.seen_cache.contains(&current_node.path_node) {
                continue;
            }

            // add path to seen cache
            self.seen_cache.push(current_node.path_node);

            // add new heat path nodes to b_heap
            if let Some(forward_node) = current_node.step_forward(&self.map) {
                self.b_heap.push(forward_node);
            }
            if let Some(left_node) = current_node.step_left(&self.map) {
                self.b_heap.push(left_node);
            }
            if let Some(right_node) = current_node.step_right(&self.map) {
                self.b_heap.push(right_node);
            }
        }
        // b_heap should never run empty
        Err(anyhow!("b_heap run empty"))
    }
}

// solution is again inspired by HyperNeutrino
// see https://www.youtube.com/watch?v=2pDSooPLLkI

pub fn day_17() -> Result<()> {
    #[cfg(feature = "long-run-time")]
    {
        let input = include_str!("../../../../aoc_input/aoc-2023/day_17.txt");
        let mut city_map = CityMap::<NormalCrucible<X, Y>, X, Y>::new(input);
        let result_part1 = city_map.get_minimum_heat_loss()?;
        println!("result day 17 part 1: {}", result_part1);
        assert_eq!(result_part1, 1099);
        let mut city_map = CityMap::<UltraCrucible<X, Y>, X, Y>::new(input);
        let result_part2 = city_map.get_minimum_heat_loss().unwrap();
        println!("result day 17 part 2: {}", result_part2);
        assert_eq!(result_part2, 1266);
    }
    #[cfg(feature = "short-run-time")]
    {
        println!("day 17 skipped because of long run time")
    }

    Ok(())
}

#[cfg(all(test, feature = "long-run-time"))]
mod tests {

    use super::*;

    const XT: usize = 13;
    const YT: usize = 13;

    #[test]
    fn test_part1_example() {
        let input = "2413432311323\n\
                           3215453535623\n\
                           3255245654254\n\
                           3446585845452\n\
                           4546657867536\n\
                           1438598798454\n\
                           4457876987766\n\
                           3637877979653\n\
                           4654967986887\n\
                           4564679986453\n\
                           1224686865563\n\
                           2546548887735\n\
                           4322674655533";
        let mut city_map = CityMap::<NormalCrucible<XT, YT>, XT, YT>::new(input);
        println!("{}", city_map.map);
        let result_part1 = city_map.get_minimum_heat_loss().unwrap();
        println!("result day 17 example part 1: {}", result_part1);
        assert_eq!(result_part1, 102);
        let mut city_map = CityMap::<UltraCrucible<XT, YT>, XT, YT>::new(input);
        let result_part2 = city_map.get_minimum_heat_loss().unwrap();
        println!("result day 17 example part 2: {}", result_part2);
        assert_eq!(result_part2, 94);
    }
}
