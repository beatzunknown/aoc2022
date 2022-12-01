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

pub struct Input<'a> {
    text: &'a str,
}

impl<'a> Input<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

pub struct Solution<'a> {
    part1: &'a str,
    part2: &'a str,
}

impl<'a> Solution<'a> {
    pub fn new(part1: &'a str, part2: &'a str) -> Self {
        Self { part1, part2 }
    }
}

impl<'a> fmt::Display for Solution<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Day 1 solution: {}", self.part1)?;
        write!(fmt, "Day 2 solution: {}", self.part2)
    }
}

pub trait Day<'a> {
    fn solve(&self, input: &'a str, part: Part) -> &'a str;

    fn get_solutions(&self, input: &'a str) -> Solution<'a> {
        Solution::new(self.solve(input, Part::One), self.solve(input, Part::Two))
    }
}
