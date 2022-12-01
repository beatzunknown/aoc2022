use aoc2022_lib::{utils, Day, Part};

#[derive(Default)]
pub struct Day01 {
    sums: Vec<u32>,
}

impl<'a> Day<'a> for Day01 {
    fn solve(&mut self, _input: &'a str, part: Part) -> String {
        if part.is_first() {
            self.sums[0].to_string()
        } else {
            self.sums[..3].iter().sum::<u32>().to_string()
        }
    }

    fn run_preliminary_tasks(&mut self, input: &'a str) {
        let mut sums: Vec<u32> = input
            .split("\n\n")
            .map(|group| utils::parse_lines(group).unwrap().iter().sum())
            .collect();
        sums.sort();
        sums.reverse();

        self.sums = sums;
    }
}
