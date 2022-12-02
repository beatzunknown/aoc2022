use aoc2022_lib::{Day, Part};

#[derive(Default)]
pub struct Day02 {}

impl<'a> Day<'a> for Day02 {
    fn solve(&mut self, input: &'a str, part: Part) -> String {
        input
            .lines()
            .map(|l| {
                let mut chars = l.chars();
                let a = chars.next().unwrap() as u32 - 'A' as u32 + 1;
                let b = chars.nth(1).unwrap() as u32 - 'X' as u32 + 1;
                match part {
                    Part::One => (b + 3 - a + 1).rem_euclid(3) * 3 + b,
                    Part::Two => ((b - 1) * 3) + (a + b).rem_euclid(3) + 1,
                }
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>()
            .to_string()
    }
}
