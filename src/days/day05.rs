use aoc2022_lib::{Day, Part};
use text_io::scan;

#[derive(Default)]
pub struct Day05 {}

impl<'a> Day<'a> for Day05 {
    fn solve(&mut self, input: &'a str, part: Part) -> String {
        let (crates, moves) = input.split_once("\n\n").unwrap();
        let mut crates_iter = crates.lines().rev();
        let col_str = crates_iter.next().unwrap();
        let n_stacks: usize = (col_str.len() as f32 / 4_f32).ceil() as usize + 1;

        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n_stacks + 1];
        crates_iter.for_each(|row| {
            let mut row = row.chars();
            // cursed input indexing lol
            let mut c = row.nth(1);
            for (i, _) in (1..col_str.len()).step_by(4).enumerate() {
                if !c.unwrap().is_whitespace() {
                    stacks[i + 1].push(c.unwrap());
                }
                c = row.nth(3);
            }
        });

        moves.lines().for_each(|l| {
            let (a, b, c): (usize, usize, usize);
            scan!(l.bytes() => "move {} from {} to {}", a, b, c);
            let new_len = stacks[b].len() - a;
            let to_move: Vec<char> = match part {
                Part::One => stacks[b].drain(new_len..).rev().collect(),
                Part::Two => stacks[b].drain(new_len..).collect(),
            };
            stacks[c].extend(to_move);
        });

        let mut ret = String::new();
        for stack in stacks.iter().skip(1) {
            if let Some(c) = stack.last() {
                ret.push(*c);
            }
        }
        ret
    }
}
