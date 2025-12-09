//!day_02.rs

use anyhow::{Result, anyhow};

#[derive(Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn max_from_str(&mut self, handful: &str) -> Result<()> {
        for cube in handful.split(',').map(|c| c.trim()) {
            let (num, ctype) = cube.split_once(' ').ok_or(anyhow!("bad input"))?;
            let num = num.parse::<u32>()?;
            match ctype {
                "red" => self.red = self.red.max(num),
                "green" => self.green = self.green.max(num),
                "blue" => self.blue = self.blue.max(num),
                _ => return Err(anyhow!("bad input")),
            }
        }
        Ok(())
    }

    fn is_possible(&self, max_cubes: &Cubes) -> bool {
        self.red <= max_cubes.red && self.green <= max_cubes.green && self.blue <= max_cubes.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn day_02() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_02.txt");
    let max_cubes = Cubes::new(12, 13, 14);
    let mut result = 0;
    let mut power = 0;
    for line in input.lines() {
        let mut cubes = Cubes::default();
        let (game, handfuls) = line.split_once(':').ok_or(anyhow!("bad input"))?;
        let game = game[5..].parse::<u32>()?;
        for handful in handfuls.split(';') {
            cubes.max_from_str(handful)?;
        }
        if cubes.is_possible(&max_cubes) {
            result += game;
        }
        power += cubes.power();
    }
    println!("result day 02 part 1: {}", result);
    assert_eq!(result, 2_416);
    println!("result day 02 part 2: {}", power);
    assert_eq!(power, 63_307);
    Ok(())
}
