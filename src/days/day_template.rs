#![allow(unused_imports)]
#![allow(dead_code)]

use aoc2022_lib::{utils, Day, Part};
use text_io::scan;

#[derive(Default)]
pub struct Day00 {}

impl<'a> Day<'a> for Day00 {
    fn solve(&mut self, _input: &'a str, part: Part) -> String {
        if part.is_first() {
            return String::new();
        }
        String::new()
    }
}
