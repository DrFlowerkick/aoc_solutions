//!day_22.rs

use anyhow::Result;
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl PartialOrd for Point3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point3D {
    fn cmp(&self, other: &Self) -> Ordering {
        // sort by z, x, y
        match self.z.cmp(&other.z) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match self.x.cmp(&other.x) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => self.y.cmp(&other.y),
            },
        }
    }
}

impl From<&str> for Point3D {
    fn from(value: &str) -> Self {
        let mut digits_iter = value
            .split(',')
            .map(|d| d.parse::<i32>().expect("bad Point3D digit"));
        let point = Self {
            x: digits_iter.next().unwrap(),
            y: digits_iter.next().unwrap(),
            z: digits_iter.next().unwrap(),
        };
        if digits_iter.next().is_some() {
            panic!("input Point3D too long");
        }
        point
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    a: Point3D,
    b: Point3D,
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.b.cmp(&other.b) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.a.cmp(&other.a),
        }
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (alpha, beta) = value
            .split_once('~')
            .map(|(a, b)| (Point3D::from(a), Point3D::from(b)))
            .unwrap();
        // A Brick is always a single line of cubes, therefore it can only extend in one direction.
        // In case of at least two cubes, make sure that a is always < b. In case of a single cube a == b.
        let (a, b) = if alpha <= beta {
            (alpha, beta)
        } else {
            (beta, alpha)
        };
        Brick { a, b }
    }
}

impl Brick {
    fn is_overlapping(&self, other: &Brick) -> bool {
        // start with z, since blocks drop down z
        if self.a.z.max(other.a.z) > self.b.z.min(other.b.z) {
            return false;
        }
        if self.a.x.max(other.a.x) > self.b.x.min(other.b.x) {
            return false;
        }
        if self.a.y.max(other.a.y) > self.b.y.min(other.b.y) {
            return false;
        }
        true
    }
    fn drop_brick_by_one(&self) -> Brick {
        let mut dropped_brick = *self;
        dropped_brick.a.z -= 1;
        dropped_brick.b.z -= 1;
        dropped_brick
    }
}

fn drop_bricks(
    bricks: &[Brick],
    supporting_bricks: &mut HashMap<Brick, Vec<Brick>>,
    supported_bricks: &mut HashMap<Brick, Vec<Brick>>,
) -> Vec<Brick> {
    let mut dropped_bricks: Vec<Brick> = Vec::with_capacity(bricks.len());
    for mut brick in bricks.iter().map(|b| b.to_owned()) {
        loop {
            if brick.a.z == 1 {
                break;
            }
            let dropped_brick = brick.drop_brick_by_one();
            let mut supporting = false;
            for supporting_brick in dropped_bricks
                .iter()
                .filter(|b| b.is_overlapping(&dropped_brick))
            {
                supporting = true;
                match supporting_bricks.get_mut(supporting_brick) {
                    Some(list_of_supported_bricks) => list_of_supported_bricks.push(brick),
                    None => {
                        supporting_bricks.insert(*supporting_brick, vec![brick]);
                    }
                }
                match supported_bricks.get_mut(&brick) {
                    Some(list_of_supporting_bricks) => {
                        list_of_supporting_bricks.push(*supporting_brick)
                    }
                    None => {
                        supported_bricks.insert(brick, vec![*supporting_brick]);
                    }
                }
            }
            if supporting {
                break;
            }
            brick = dropped_brick;
        }
        dropped_bricks.push(brick);
    }
    dropped_bricks
}

fn bricks_to_disintegrate_without_consequence(
    bricks: &[Brick],
    supporting_bricks: &HashMap<Brick, Vec<Brick>>,
    supported_bricks: &HashMap<Brick, Vec<Brick>>,
) -> Vec<Brick> {
    let mut bricks_save_to_disintegrate: Vec<Brick> = Vec::with_capacity(bricks.len());
    // bricks in between, which support other bricks, who are supported by at least one more brick
    for (supporting_brick, list_of_supported_bricks) in supporting_bricks.iter() {
        if list_of_supported_bricks
            .iter()
            .all(|b| supported_bricks.get(b).unwrap().len() > 1)
        {
            bricks_save_to_disintegrate.push(*supporting_brick);
        }
    }
    // bricks on top which do not support
    for supported_brick in supported_bricks.keys() {
        if supporting_bricks.get(supported_brick).is_none() {
            bricks_save_to_disintegrate.push(*supported_brick);
        }
    }
    // bricks on the ground, which do not support
    for brick in bricks.iter().filter(|b| {
        !supporting_bricks.keys().any(|k| k == *b) && !supported_bricks.keys().any(|k| k == *b)
    }) {
        bricks_save_to_disintegrate.push(*brick);
    }
    bricks_save_to_disintegrate
}

fn bricks_to_disintegrate_with_consequence(
    bricks: &[Brick],
    bricks_save_to_disintegrate: &[Brick],
    supporting_bricks: &HashMap<Brick, Vec<Brick>>,
    supported_bricks: &HashMap<Brick, Vec<Brick>>,
    falling_bricks: &mut HashMap<Brick, Vec<Brick>>,
) -> usize {
    // start from top to fill falling_bricks from top to down
    for brick in bricks.iter().rev() {
        if bricks_save_to_disintegrate.contains(brick) {
            // skip bricks which are save to disintegrate
            continue;
        }

        let mut bricks_to_fall: Vec<Brick> = vec![*brick];
        let mut bricks_to_check: VecDeque<Brick> = VecDeque::new();
        bricks_to_check.push_back(*brick);
        while let Some(falling_brick) = bricks_to_check.pop_front() {
            if let Some(bricks_supported_by_falling_brick) = supporting_bricks.get(&falling_brick) {
                for brick_to_fall in bricks_supported_by_falling_brick.iter() {
                    // brick_to_fall could already be in bricks_to_fall, if it is carried by multiple bricks, which fall together
                    if bricks_to_fall.contains(brick_to_fall) {
                        continue;
                    }
                    // hanging free, if all supporters of brick_to_fall are in bricks_to_fall
                    let is_hanging_free = supported_bricks
                        .get(brick_to_fall)
                        .unwrap()
                        .iter()
                        .all(|sb| bricks_to_fall.contains(sb));
                    if is_hanging_free {
                        bricks_to_fall.push(*brick_to_fall);
                        bricks_to_check.push_back(*brick_to_fall);
                    }
                }
            }
        }
        // add result to cache, skip first element, since it is the brick to disintegrate
        falling_bricks.insert(*brick, bricks_to_fall[1..].to_vec());
    }

    // sum of all falling bricks is result of part 2
    falling_bricks.values().map(|fb| fb.len()).sum()
}

pub fn day_22() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_22.txt");
    let mut supporting_bricks: HashMap<Brick, Vec<Brick>> = HashMap::new();
    let mut supported_bricks: HashMap<Brick, Vec<Brick>> = HashMap::new();
    let mut falling_bricks: HashMap<Brick, Vec<Brick>> = HashMap::new();
    let mut bricks: Vec<Brick> = input.lines().map(Brick::from).collect();
    bricks.sort();

    bricks = drop_bricks(&bricks, &mut supporting_bricks, &mut supported_bricks);

    let bricks_save_to_disintegrate =
        bricks_to_disintegrate_without_consequence(&bricks, &supporting_bricks, &supported_bricks);
    println!(
        "result day 22 part 1: {}",
        bricks_save_to_disintegrate.len()
    );
    assert_eq!(bricks_save_to_disintegrate.len(), 471);

    let result_part2 = bricks_to_disintegrate_with_consequence(
        &bricks,
        &bricks_save_to_disintegrate,
        &supporting_bricks,
        &supported_bricks,
        &mut falling_bricks,
    );
    println!("result day 22 part 2: {}", result_part2);
    assert_eq!(result_part2, 68_525);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part1() -> Result<()> {
        let input = "1,0,1~1,2,1\n\
                           0,0,2~2,0,2\n\
                           0,2,3~2,2,3\n\
                           0,0,4~0,2,4\n\
                           2,0,5~2,2,5\n\
                           0,1,6~2,1,6\n\
                           1,1,8~1,1,9";
        let mut supporting_bricks: HashMap<Brick, Vec<Brick>> = HashMap::new();
        let mut supported_bricks: HashMap<Brick, Vec<Brick>> = HashMap::new();
        let mut falling_bricks: HashMap<Brick, Vec<Brick>> = HashMap::new();
        let mut bricks: Vec<Brick> = input.lines().map(Brick::from).collect();
        bricks.sort();

        assert!(bricks[0].is_overlapping(&bricks[1].drop_brick_by_one()));
        assert!(!bricks[1].is_overlapping(&bricks[2].drop_brick_by_one()));

        eprintln!("sorted bricks");
        for brick in bricks.iter() {
            eprintln!("{:?}", brick);
        }

        bricks = drop_bricks(&bricks, &mut supporting_bricks, &mut supported_bricks);
        eprintln!("dropped bricks");
        for brick in bricks.iter() {
            eprintln!("{:?}", brick);
        }

        eprintln!("bricks who support");
        for (brick, bricks_to_support) in supporting_bricks.iter() {
            eprintln!("supporting brick: {:?}", brick);
            for bts in bricks_to_support.iter() {
                eprintln!("    supported brick: {:?}", bts);
            }
        }
        eprintln!("bricks who are supported");
        for (brick, bricks_who_support) in supported_bricks.iter() {
            eprintln!("supported brick: {:?}", brick);
            for bws in bricks_who_support.iter() {
                eprintln!("    supporting brick: {:?}", bws);
            }
        }

        eprintln!("bricks which could be disintegrated");
        let bricks_save_to_disintegrate = bricks_to_disintegrate_without_consequence(
            &bricks,
            &supporting_bricks,
            &supported_bricks,
        );
        for disintegrate_brick in bricks_save_to_disintegrate.iter() {
            eprintln!("{:?}", disintegrate_brick);
        }
        println!(
            "result day 22 example part 1: {}",
            bricks_save_to_disintegrate.len()
        );
        assert_eq!(bricks_save_to_disintegrate.len(), 5);

        eprintln!("bricks which fall if one brick is disintegrated");
        let result_part2 = bricks_to_disintegrate_with_consequence(
            &bricks,
            &bricks_save_to_disintegrate,
            &supporting_bricks,
            &supported_bricks,
            &mut falling_bricks,
        );
        for (disintegrated_brick, bricks_to_fall) in falling_bricks.iter() {
            eprintln!("disintegrated_brick: {:?}", disintegrated_brick);
            for falling_brick in bricks_to_fall.iter() {
                eprintln!("    falling_brick: {:?}", falling_brick);
            }
        }
        println!("result day 22 example part 2: {}", result_part2);
        assert_eq!(result_part2, 7);

        Ok(())
    }
}
