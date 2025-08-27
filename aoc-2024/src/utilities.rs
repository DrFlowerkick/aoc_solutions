//! Some useful utilities.

use std::collections::HashSet;

pub struct SnapshotHashSet<T> {
    set: HashSet<T>,
    baseline: Option<HashSet<T>>,
}

impl<T: std::hash::Hash + Eq + Clone> SnapshotHashSet<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            set: HashSet::with_capacity(capacity),
            baseline: None,
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }

    pub fn set_baseline(&mut self) {
        self.baseline = Some(self.set.clone());
    }

    pub fn reset_to_baseline(&mut self) {
        if let Some(baseline) = self.baseline.take() {
            self.set = baseline;
        }
    }
}

pub fn digit_count(mut num: u128) -> u32 {
    let mut count = 1;
    loop {
        num /= 10;
        if num == 0 {
            break;
        }
        count += 1;
    }
    count
}
