use aoc2022_lib::{Day, Part};
use text_io::scan;

#[derive(Default)]
pub struct Day04 {}

impl<'a> Day<'a> for Day04 {
    fn solve(&mut self, input: &'a str, part: Part) -> String {
        input
            .lines()
            .map(|l| {
                let (a, b, c, d): (u32, u32, u32, u32);
                scan!(l.bytes() => "{}-{},{}-{}", a, b, c, d);
                if part.is_first() {
                    // 1st fully in 2nd or vice versa
                    ((a <= c && b >= d) || (c <= a && d >= b)) as u32
                } else {
                    // 1st range starts in 2nd or vice versa
                    ((a <= d) && (a >= c) || ((c <= b) && (c >= a))) as u32
                }
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>()
            .to_string()
    }
}
