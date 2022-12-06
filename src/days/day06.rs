use aoc2022_lib::{Day, Part};
use std::collections::HashSet;

#[derive(Default)]
pub struct Day06 {}

impl<'a> Day<'a> for Day06 {
    fn solve(&mut self, input: &'a str, part: Part) -> String {
        let chars = input.trim().as_bytes();
        let n = match part {
            Part::One => 4,
            Part::Two => 14,
        };

        let mut i = n;
        while HashSet::<&u8>::from_iter(chars[(i - n)..i].iter()).len() < n {
            i += 1;
        }
        i.to_string()
    }
}
