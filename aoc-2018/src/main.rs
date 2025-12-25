//!main.rs

use aoc_2018::run;

fn main() {
    if let Err(err) = run() {
        println!("Error occurred: {err}");

        // look for source
        if let Some(source) = err.source() {
            println!("Source of error: {source:?}");
        }
    }
}
