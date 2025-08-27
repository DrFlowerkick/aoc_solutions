//!day_13.rs

use anyhow::Result;
use my_lib::my_map_two_dim::MyMap2D;

// max values for X and Y over all patterns taken from ../../../../aoc_input/aoc-2023/day_13.txt
const X: usize = 17;
const Y: usize = 17;

#[derive(PartialEq, Eq, Clone, Copy, Default)]
enum Cell {
    #[default]
    None,
    Ash,
    Rock,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Ash,
            '#' => Cell::Rock,
            _ => panic!("bad Cell char"),
        }
    }
}

enum MirrorResult {
    None,
    Smudge,
    Clean,
}

#[derive(Clone, Copy)]
struct Pattern<const X: usize, const Y: usize> {
    pat: MyMap2D<Cell, X, Y>,
    mirror_at: usize, // index of right column respectively bottom row, must be > 0 to be valid
    mirror_axis: bool, // false: row, true: column, valid, if mirror_at is valid
    mirror_value: usize,
    smudge_mirror_at: usize, // index of right column respectively bottom row, must be > 0 to be valid
    smudge_mirror_axis: bool, // false: row, true: column, valid, if mirror_at is valid
    smudge_mirror_value: usize,
}

impl<const X: usize, const Y: usize> Pattern<X, Y> {
    fn new(value: &str) -> Self {
        let mut pattern = Pattern {
            pat: value.into(),
            mirror_at: 0,
            mirror_axis: false,
            mirror_value: 0,
            smudge_mirror_at: 0,
            smudge_mirror_axis: false,
            smudge_mirror_value: 0,
        };
        pattern.set_mirror_axis();
        pattern
    }
    fn set_mirror_axis(&mut self) {
        let mut clean = false;
        let mut smudge = false;
        // first try to find mirro axis in rows ...
        for r1 in 0..Y - 1 {
            let r2 = r1 + 1;
            if !self.valid_row(r2) {
                break;
            }
            match self.check_mirror_row(r1, r2) {
                MirrorResult::None => (),
                MirrorResult::Clean => clean = true,
                MirrorResult::Smudge => smudge = true,
            }
            if clean && smudge {
                return;
            }
        }
        // .. than try to find mirro axis in columns
        for c1 in 0..X - 1 {
            let c2 = c1 + 1;
            if !self.valid_column(c2) {
                break;
            }
            match self.check_mirror_column(c1, c2) {
                MirrorResult::None => (),
                MirrorResult::Clean => clean = true,
                MirrorResult::Smudge => smudge = true,
            }
            if clean && smudge {
                return;
            }
        }
        if !clean {
            panic!("did not find a clean mirror axis ")
        }
        if !smudge {
            panic!("did not find a smudge mirror axis ")
        }
    }
    fn valid_row(&self, row: usize) -> bool {
        // check is required, since map can be greater than provided pattern
        *self.pat.get((0, row).into()) != Cell::None
    }
    fn valid_column(&self, col: usize) -> bool {
        // check is required, since map can be greater than provided pattern
        *self.pat.get((col, 0).into()) != Cell::None
    }
    fn check_mirror_row(&mut self, r1: usize, r2: usize) -> MirrorResult {
        let mut bottom = r2; // row[Y - 1] is bottom most row
        let mut top = r1; // row[0] is top most row
        let mut smudge = false;
        loop {
            match self
                .pat
                .iter_row(top)
                .map(|(_, v)| v)
                .zip(self.pat.iter_row(bottom).map(|(_, v)| v))
                .filter(|(v1, v2)| v1 != v2)
                .count()
            {
                1 => {
                    if smudge {
                        return MirrorResult::None;
                    }
                    smudge = true;
                }
                0 => (),
                _ => return MirrorResult::None,
            }
            if top == 0 || bottom + 1 == Y || !self.valid_row(bottom + 1) {
                break;
            }
            top -= 1;
            bottom += 1;
        }
        if smudge {
            // found smufge mirror row
            self.smudge_mirror_at = r2;
            self.smudge_mirror_axis = false;
            self.smudge_mirror_value = r2 * 100;
            MirrorResult::Smudge
        } else {
            // found clean mirror row
            self.mirror_at = r2;
            self.mirror_axis = false;
            self.mirror_value = r2 * 100;
            MirrorResult::Clean
        }
    }
    fn check_mirror_column(&mut self, c1: usize, c2: usize) -> MirrorResult {
        let mut right = c2; // row[Y - 1] is right most column
        let mut left = c1; // row[0] is left most row
        let mut smudge = false;
        loop {
            match self
                .pat
                .iter_column(left)
                .map(|(_, v)| v)
                .zip(self.pat.iter_column(right).map(|(_, v)| v))
                .filter(|(v1, v2)| v1 != v2)
                .count()
            {
                1 => {
                    if smudge {
                        return MirrorResult::None;
                    }
                    smudge = true;
                }
                0 => (),
                _ => return MirrorResult::None,
            }
            if left == 0 || right + 1 == Y || !self.valid_column(right + 1) {
                break;
            }
            left -= 1;
            right += 1;
        }
        if smudge {
            // found smufge mirror row
            self.smudge_mirror_at = c2;
            self.smudge_mirror_axis = false;
            self.smudge_mirror_value = c2;
            MirrorResult::Smudge
        } else {
            // found clean mirror row
            self.mirror_at = c2;
            self.mirror_axis = false;
            self.mirror_value = c2;
            MirrorResult::Clean
        }
    }
}

pub fn day_13() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_13.txt");
    let mut result_part1 = 0;
    let mut result_part2 = 0;
    for pat in input.split("\n\n") {
        let pattern = Pattern::<X, Y>::new(pat);
        result_part1 += pattern.mirror_value;
        result_part2 += pattern.smudge_mirror_value;
    }

    println!("result day 13 part 1: {}", result_part1);
    assert_eq!(result_part1, 33_735);
    println!("result day 13 part 2: {}", result_part2);
    assert_eq!(result_part2, 38_063);

    Ok(())
}
