use aoc2022_lib::{utils, Day, Part};
use std::collections::HashSet;

#[derive(Default)]
pub struct Day03 {}

impl<'a> Day<'a> for Day03 {
    fn solve(&mut self, input: &'a str, part: Part) -> String {
        if part.is_first() {
            input
                .lines()
                .map(|sack| {
                    let (a, b) = sack.split_at(sack.len() / 2);
                    get_shared_item_priority(&[a, b])
                })
                .collect::<Vec<u32>>()
                .iter()
                .sum::<u32>()
                .to_string()
        } else {
            Vec::from_iter(input.lines())
                .chunks(3)
                .map(get_shared_item_priority)
                .collect::<Vec<u32>>()
                .iter()
                .sum::<u32>()
                .to_string()
        }
    }
}

fn get_shared_item_priority(sacks: &[&str]) -> u32 {
    let common = *utils::intersect_sets(sacks.iter().map(|x| HashSet::from_iter(x.chars())))
        .unwrap()
        .iter()
        .next()
        .unwrap();
    if common.is_uppercase() {
        common as u32 - 'A' as u32 + 27
    } else {
        common as u32 - 'a' as u32 + 1
    }
}
