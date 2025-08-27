//!day_12.rs

use std::collections::HashMap;

use anyhow::Result;

fn springs_and_damaged_clusters(input: &str) -> (&str, Vec<usize>) {
    input
        .trim()
        .split_once(' ')
        .map(|(springs, d)| {
            if springs
                .trim()
                .chars()
                .any(|c| !['.', '#', '?'].contains(&c))
            {
                panic!("bad spring char");
            }
            let damaged_clusters: Vec<usize> = d
                .trim()
                .split(',')
                .map(|s| s.parse::<usize>().expect("bad input"))
                .collect();
            (springs.trim(), damaged_clusters)
        })
        .unwrap()
}

fn unfold_springs_and_damaged_clusters(
    springs: &str,
    damaged_clusters: &[usize],
) -> (String, Vec<usize>) {
    let mut unfolded_springs = springs.to_string();
    let mut unfolded_damaged_clusters = damaged_clusters.to_vec();
    for _i in 0..4 {
        unfolded_springs = unfolded_springs + "?" + springs;
        unfolded_damaged_clusters.extend_from_slice(damaged_clusters);
    }
    (unfolded_springs, unfolded_damaged_clusters)
}

// this solution is an implementation in Rust of https://www.youtube.com/watch?v=g3Ms5e7Jdqo by HyperNeutrino
// www.youtube.com/@hyper-neutrino
// github.com/hyper-neutrino
fn different_arrangements(
    springs: &str,
    damaged_clusters: &[usize],
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    // no more springs left
    if springs.is_empty() {
        if damaged_clusters.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }
    // no more damaged clusters left
    if damaged_clusters.is_empty() {
        if springs.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    // if in cache, returned cached result
    if let Some(cached_num_different_arrangements) =
        cache.get(&(springs.to_string(), damaged_clusters.to_vec()))
    {
        return *cached_num_different_arrangements;
    }

    // search for different configurations
    let mut num_different_arrangements = 0;

    // pretend '?' as operational spring
    if springs[0..1].contains(['.', '?']) {
        num_different_arrangements +=
            different_arrangements(&springs[1..], damaged_clusters, cache);
    }

    // pretend '?' as damaged spring
    if springs[0..1].contains(['#', '?'])
        && damaged_clusters[0] <= springs.chars().count()
        && !springs[..damaged_clusters[0]].contains('.')
    {
        if damaged_clusters[0] == springs.chars().count() {
            // if end of springs, call different_arrangements with "" to check len() of damaged_clusters
            num_different_arrangements += different_arrangements("", &damaged_clusters[1..], cache)
        } else if springs.chars().nth(damaged_clusters[0]).unwrap() != '#' {
            // if remaining springs, call different_arrangements with remaining springs and damaged_clusters
            num_different_arrangements += different_arrangements(
                &springs[(damaged_clusters[0] + 1)..],
                &damaged_clusters[1..],
                cache,
            );
        }
    }
    // insert current result into cache for later use
    cache.insert(
        (springs.to_string(), damaged_clusters.to_vec()),
        num_different_arrangements,
    );

    num_different_arrangements
}

pub fn day_12() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_12.txt");
    let mut sum_different_arrangements = 0;
    let mut sum_different_arrangements_unfolded = 0;
    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    for line in input.lines() {
        let (springs, damaged_clusters) = springs_and_damaged_clusters(line);
        sum_different_arrangements +=
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        let (unfolded_springs, unfolded_damaged_clusters) =
            unfold_springs_and_damaged_clusters(springs, &damaged_clusters);
        sum_different_arrangements_unfolded += different_arrangements(
            unfolded_springs.as_str(),
            &unfolded_damaged_clusters[..],
            &mut cache,
        );
    }

    println!("result day 12 part 1: {}", sum_different_arrangements);
    assert_eq!(sum_different_arrangements, 7_460);
    println!(
        "result day 12 part 2: {}",
        sum_different_arrangements_unfolded
    );
    assert_eq!(sum_different_arrangements_unfolded, 6_720_660_274_964);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_arrangements() {
        let input = "???.### 1,1,3";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 01 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 1);
    }

    #[test]
    fn test_arrangements_02() {
        let input = ".??..??...?##. 1,1,3";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 02 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 4);
    }

    #[test]
    fn test_arrangements_03() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 03 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 1);
    }

    #[test]
    fn test_arrangements_04() {
        let input = "????.#...#... 4,1,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 04 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 1);
    }

    #[test]
    fn test_arrangements_05() {
        let input = "????.######..#####. 1,6,5";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 05 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 4);
    }

    #[test]
    fn test_arrangements_06() {
        let input = "?###???????? 3,2,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 06 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 10);
    }

    #[test]
    fn test_arrangements_07() {
        let input = ".#??#..???#..?? 2,2,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 07 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 0);
    }

    #[test]
    fn test_arrangements_08() {
        let input = ".#??#..???#..?? 2,1,2";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 08 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 1);
    }

    #[test]
    fn test_arrangements_09() {
        let input = "????#?.??? 2,1,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 09 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 8);
    }

    #[test]
    fn test_arrangements_10() {
        let input = "??.??????#???#?????# 1,1,7,3,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 10 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 9);
    }

    #[test]
    fn test_arrangements_11() {
        let input = ".##.?#??.#.?# 2,1,1,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(springs, &damaged_clusters[..], &mut cache);
        println!(
            "test 11 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 1);
    }
}
