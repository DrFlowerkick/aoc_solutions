//!day_23.rs

use anyhow::Result;
use my_lib::{
    my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D, my_tree::TreeNode,
};
use std::{collections::HashMap, rc::Rc};

// values taken from ../../../../aoc_input/aoc-2023/day_23.txt
const X: usize = 141;
const Y: usize = 141;

#[derive(PartialEq)]
struct Node<const X: usize, const Y: usize> {
    start_point: MapPoint<X, Y>,
    end_point: MapPoint<X, Y>,
    next_points: Vec<MapPoint<X, Y>>,
    steps: usize,
    total_steps: usize,
}

impl<const X: usize, const Y: usize> Node<X, Y> {
    fn new(
        start_point: MapPoint<X, Y>,
        last_point: MapPoint<X, Y>,
        maze: &Maze<X, Y>,
        previous_steps: usize,
        cache: &mut HashMap<
            (MapPoint<X, Y>, MapPoint<X, Y>),
            (MapPoint<X, Y>, Vec<MapPoint<X, Y>>, usize),
        >,
    ) -> Self {
        if let Some((cached_end_point, cached_next_points, cached_steps)) =
            cache.get(&(start_point, last_point))
        {
            return Self {
                start_point,
                end_point: *cached_end_point,
                next_points: cached_next_points.clone(),
                steps: *cached_steps,
                total_steps: previous_steps + cached_steps,
            };
        }
        let offset = if start_point == last_point {
            0
        } else if start_point
            .iter_neighbors(Compass::N, true, false, false)
            .any(|(n, _)| n == last_point)
        {
            1
        } else {
            panic!("last_point is not start_point or neighbor of start_point");
        };
        let (steps, (end_point, next_points)) =
            MazeRunner::<X, Y>::new(maze, start_point, last_point)
                .enumerate()
                .last()
                .unwrap();
        cache.insert(
            (start_point, last_point),
            (end_point, next_points.clone(), steps + offset),
        );
        Self {
            start_point,
            end_point,
            next_points,
            steps: steps + offset,
            total_steps: steps + previous_steps + offset,
        }
    }
}

struct Maze<const X: usize, const Y: usize> {
    maze: MyMap2D<char, X, Y>,
    start_point: MapPoint<X, Y>,
    end_point: MapPoint<X, Y>,
    climbing_is_possible: bool,
}

impl<const X: usize, const Y: usize> From<&str> for Maze<X, Y> {
    fn from(value: &str) -> Self {
        Maze {
            maze: MyMap2D::from(value),
            start_point: (1, 0).into(),
            end_point: (X - 2, Y - 1).into(),
            climbing_is_possible: false,
        }
    }
}

impl<const X: usize, const Y: usize> Maze<X, Y> {
    fn is_possible(&self, point: &MapPoint<X, Y>, orientation: &Compass) -> bool {
        match self.maze.get(*point) {
            '.' => true,
            '^' => *orientation == Compass::N || self.climbing_is_possible,
            '>' => *orientation == Compass::E || self.climbing_is_possible,
            'v' => *orientation == Compass::S || self.climbing_is_possible,
            '<' => *orientation == Compass::W || self.climbing_is_possible,
            _ => false,
        }
    } /*
      fn next_possible_points(&self, point: &MapPoint<X, Y>) -> Vec<MapPoint<X, Y>> {
          self.maze
              .iter_neighbors(*point)
              .filter(|(p, o, v)| **v != '.' && **v != '#' && self.is_possible(p, o))
              .map(|(p, ..)| p)
              .collect()
      } */
    fn go_hiking(&self) -> usize {
        let mut cache: HashMap<
            (MapPoint<X, Y>, MapPoint<X, Y>),
            (MapPoint<X, Y>, Vec<MapPoint<X, Y>>, usize),
        > = HashMap::new();
        let root_node = Node::<X, Y>::new(self.start_point, self.start_point, self, 0, &mut cache);
        let hiking_tree: Rc<TreeNode<Node<X, Y>>> = TreeNode::seed_root(root_node, 3);
        for (hiking_node, _) in hiking_tree.iter_level_order_traversal() {
            for next_start_point in hiking_node.get_value().next_points.iter() {
                let next_hiking_node = Node::<X, Y>::new(
                    *next_start_point,
                    hiking_node.get_value().end_point,
                    self,
                    hiking_node.get_value().total_steps,
                    &mut cache,
                );
                if !hiking_node
                    .iter_back_track()
                    .any(|v| v[0].get_value().end_point == next_hiking_node.end_point)
                {
                    // add hiking_node, if endpoint has not already been visited
                    hiking_node.add_child(next_hiking_node, 3);
                }
            }
        }
        // longest path is hiking node with most steps
        hiking_tree
            .iter_pre_order_traversal()
            .filter(|n| n.get_value().end_point == self.end_point)
            .map(|n| n.get_value().total_steps)
            .max()
            .unwrap()
    }
}

struct MazeRunner<'a, const X: usize, const Y: usize> {
    maze: &'a Maze<X, Y>,
    current_point: MapPoint<X, Y>,
    last_point: MapPoint<X, Y>,
    finished: bool,
}

impl<'a, const X: usize, const Y: usize> MazeRunner<'a, X, Y> {
    fn new(maze: &'a Maze<X, Y>, start_point: MapPoint<X, Y>, last_point: MapPoint<X, Y>) -> Self {
        MazeRunner {
            maze,
            current_point: start_point,
            last_point,
            finished: false,
        }
    }
}

impl<'a, const X: usize, const Y: usize> Iterator for MazeRunner<'a, X, Y> {
    type Item = (MapPoint<X, Y>, Vec<MapPoint<X, Y>>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let result: Self::Item = (
            self.current_point,
            self.maze
                .maze
                .iter_neighbors(self.current_point)
                .filter(|(p, o, _)| *p != self.last_point && self.maze.is_possible(p, o))
                .map(|(p, ..)| p)
                .collect(),
        );
        self.last_point = self.current_point;

        if result.1.len() == 1 {
            self.current_point = result.1[0];
        } else {
            self.finished = true;
        }
        Some(result)
    }
}

pub fn day_23() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_23.txt");
    let mut maze = Maze::<X, Y>::from(input);
    let result_part1 = maze.go_hiking();
    println!("result day 23 part 1: {}", result_part1);
    maze.climbing_is_possible = true;
    #[cfg(feature = "long-run-time")]
    {
        let result_part2 = maze.go_hiking();
        println!("result day 23 part 2: {}", result_part2);
        assert_eq!(result_part2, 6322);
    }
    #[cfg(feature = "short-run-time")]
    {
        println!("day 23 part 2 skipped because of long run time")
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const XT: usize = 23;
    const YT: usize = 23;

    #[test]
    fn test_example_part1() -> Result<()> {
        let input = "#.#####################\n\
                           #.......#########...###\n\
                           #######.#########.#.###\n\
                           ###.....#.>.>.###.#.###\n\
                           ###v#####.#v#.###.#.###\n\
                           ###.>...#.#.#.....#...#\n\
                           ###v###.#.#.#########.#\n\
                           ###...#.#.#.......#...#\n\
                           #####.#.#.#######.#.###\n\
                           #.....#.#.#.......#...#\n\
                           #.#####.#.#.#########v#\n\
                           #.#...#...#...###...>.#\n\
                           #.#.#v#######v###.###v#\n\
                           #...#.>.#...>.>.#.###.#\n\
                           #####v#.#.###v#.#.###.#\n\
                           #.....#...#...#.#.#...#\n\
                           #.#########.###.#.#.###\n\
                           #...###...#...#...#.###\n\
                           ###.###.#.###v#####v###\n\
                           #...#...#.#.>.>.#.>.###\n\
                           #.###.###.#.###.#.#v###\n\
                           #.....###...###...#...#\n\
                           #####################.#";
        let mut maze = Maze::<XT, YT>::from(input);
        let result_part1 = maze.go_hiking();
        println!("result day 23 example part 1: {}", result_part1);
        assert_eq!(result_part1, 94);
        maze.climbing_is_possible = true;
        let result_part2 = maze.go_hiking();
        println!("result day 23 example part 2: {}", result_part2);
        assert_eq!(result_part2, 154);

        Ok(())
    }
}
