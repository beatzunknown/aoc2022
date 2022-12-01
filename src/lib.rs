pub mod utils;

use std::fmt;

pub const INPUT_PREFIX: &str = "src/days/day";
pub const INPUT_SUFFIX: &str = "_input.txt";

pub enum Part {
    One,
    Two,
}

impl Part {
    pub fn is_first(&self) -> bool {
        matches!(self, Part::One)
    }
}

pub struct Solution {
    part1: String,
    part2: String,
}

impl Solution {
    pub fn new(part1: String, part2: String) -> Self {
        Self { part1, part2 }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "Day 1 solution: {}", self.part1)?;
        write!(fmt, "Day 2 solution: {}", self.part2)
    }
}

pub trait Day<'a> {
    fn solve(&mut self, input: &'a str, part: Part) -> String;

    fn run_preliminary_tasks(&mut self, _input: &'a str) {}

    fn get_solutions(&mut self, input: &'a str) -> Solution {
        self.run_preliminary_tasks(input);
        Solution::new(self.solve(input, Part::One), self.solve(input, Part::Two))
    }
}
