//!day_09.rs

use anyhow::Result;

#[derive(Debug, Clone)]
struct Day09Data {
    blocks: Vec<Option<usize>>,
    max_block_id: usize,
}

impl From<&str> for Day09Data {
    fn from(value: &str) -> Self {
        let mut is_block = true;
        let mut block_id = 0;
        let mut blocks: Vec<Option<usize>> = Vec::new();
        for size in value
            .chars()
            .filter_map(|s| s.to_digit(10))
            .map(|n| n as usize)
        {
            let block = if is_block {
                let block = Some(block_id);
                block_id += 1;
                block
            } else {
                None
            };
            let mut new_block_range = vec![block; size];
            blocks.append(&mut new_block_range);
            is_block = !is_block;
        }
        Self {
            blocks,
            max_block_id: block_id - 1,
        }
    }
}

impl Day09Data {
    fn compacting_blocks(&mut self) {
        let mut start_index = 0;
        let mut end_index = self.blocks.len() - 1;
        while start_index < end_index {
            if self.blocks[start_index].is_some() {
                start_index += 1;
                continue;
            }
            if self.blocks[end_index].is_none() {
                end_index -= 1;
                continue;
            }
            self.blocks.swap(start_index, end_index);
        }
    }
    fn compacting_files(&mut self) {
        let mut next_id = self.max_block_id;
        let mut start_index_block = self.blocks.len();
        let mut current_block_size;
        let mut first_free_index = 0;
        let mut start_index_free;
        loop {
            // identify next block to proceed
            (start_index_block, current_block_size) =
                match self.identify_next_block_to_process(start_index_block, next_id) {
                    Some(block_indices) => block_indices,
                    None => return, // no more blocks to process
                };
            if start_index_block < first_free_index {
                // no more space to swap blocks
                return;
            }
            next_id -= 1;
            // find free block chunk big enough for current id block
            (first_free_index, start_index_free) = match self.identify_free_fitting_blocks(
                first_free_index,
                current_block_size,
                start_index_block,
            ) {
                Some(free_indices) => free_indices,
                None => continue, // could not find fitting block; move on to next block
            };
            // found free block! Let's swap
            let (left, right) = self.blocks[..].split_at_mut(start_index_block);
            left[start_index_free..start_index_free + current_block_size]
                .swap_with_slice(&mut right[..current_block_size]);
        }
    }
    fn identify_next_block_to_process(
        &self,
        last_start_index_block: usize,
        next_id: usize,
    ) -> Option<(usize, usize)> {
        let end_index_block = match self.blocks[..last_start_index_block]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, b)| match b {
                Some(block_id) => *block_id == next_id,
                None => false,
            }) {
            Some((ebi, _)) => ebi,
            None => return None, // all remaining blocks have been processed
        };
        // get start index of block
        let start_index_block = match self.blocks[..end_index_block]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, b)| match b {
                Some(block_id) => *block_id != next_id,
                None => true,
            }) {
            Some((sbi, _)) => sbi + 1, // increment to get index of start of current ID block
            None => 0, // this should never happen, but 0 is correct for first block in block sequence
        };
        Some((start_index_block, end_index_block - start_index_block + 1))
    }

    fn identify_free_fitting_blocks(
        &self,
        last_first_free_index: usize,
        min_size: usize,
        start_index_block: usize,
    ) -> Option<(usize, usize)> {
        let first_free_index = match self.blocks[last_first_free_index] {
            Some(_) => match self.blocks[last_first_free_index..]
                .iter()
                .position(|b| b.is_none())
            {
                Some(pos) => last_first_free_index + pos,
                None => return None,
            },
            None => last_first_free_index,
        };
        let mut start_index_free = first_free_index;
        loop {
            let start_index_next_block_after_free = match self.blocks[start_index_free..]
                .iter()
                .position(|b| b.is_some())
            {
                Some(pos) => start_index_free + pos,
                None => return None, // could not find any more blocks and therefore we must be right of start_index_block
            };
            if start_index_next_block_after_free > start_index_block {
                // could not find enough free space left of current block
                return None;
            }
            if start_index_next_block_after_free - start_index_free >= min_size {
                return Some((first_free_index, start_index_free));
            }
            start_index_free = match self.blocks[start_index_next_block_after_free..]
                .iter()
                .position(|b| b.is_none())
            {
                Some(pos) => start_index_next_block_after_free + pos,
                None => return None, // could not find any more free blocks
            };
        }
    }

    fn calc_check_sum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.as_ref().map(|ref_b| (i, ref_b)))
            .map(|(pos, block_id)| pos * block_id)
            .sum()
    }
}

pub fn day_09() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2024/day_09.txt");
    let mut challenge = Day09Data::from(input);
    let mut challenge_part2 = challenge.clone();
    challenge.compacting_blocks();

    let result_part1 = challenge.calc_check_sum();
    println!("result day 09 part 1: {}", result_part1);
    assert_eq!(result_part1, 6_307_275_788_409);

    challenge_part2.compacting_files();
    let result_part2 = challenge_part2.calc_check_sum();
    println!("result day 09 part 2: {}", result_part2);
    assert_eq!(result_part2, 6_327_174_563_252);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fmt::Display;

    impl Display for Day09Data {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for block in self.blocks.iter() {
                match block {
                    Some(id) => write!(f, "{id}")?,
                    None => write!(f, ".")?,
                }
            }
            Ok(())
        }
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2024/day_09_example.txt");
        let mut challenge = Day09Data::from(input);
        let mut challenge_part2 = challenge.clone();
        assert_eq!(
            format!("{challenge}"),
            "00...111...2...333.44.5555.6666.777.888899"
        );
        challenge.compacting_blocks();
        assert_eq!(
            format!("{challenge}"),
            "0099811188827773336446555566.............."
        );

        let result_part1 = challenge.calc_check_sum();
        println!("result day 09 part 1: {}", result_part1);
        assert_eq!(result_part1, 1_928);

        challenge_part2.compacting_files();
        assert_eq!(
            challenge_part2.to_string(),
            "00992111777.44.333....5555.6666.....8888.."
        );

        let result_part2 = challenge_part2.calc_check_sum();
        println!("result day 09 part 2: {}", result_part2);
        assert_eq!(result_part2, 2_858);

        Ok(())
    }
}
