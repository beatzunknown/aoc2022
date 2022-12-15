use aoc2022_lib::{Day, Part};
use std::cmp::Ordering;
use std::io::Error;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Default)]
pub struct Day13 {
    pairs: Vec<(VecOrInt, VecOrInt)>,
}

impl<'a> Day<'a> for Day13 {
    fn solve(&mut self, _input: &'a str, part: Part) -> String {
        if part.is_first() {
            return self
                .pairs
                .iter()
                .map(|(a, b)| VecOrInt::compare(a, b))
                .enumerate()
                .filter_map(|(i, x)| {
                    if matches!(x, Ordering::Less) {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
                .to_string();
        } else {
            let divider1: VecOrInt = "[[2]]".parse().unwrap();
            let divider2: VecOrInt = "[[6]]".parse().unwrap();

            let (mut packets, second_packets): (Vec<VecOrInt>, Vec<VecOrInt>) =
                self.pairs.clone().into_iter().unzip();
            packets.extend(second_packets);
            packets.extend([divider1.clone()]);
            packets.extend([divider2.clone()]);
            packets.sort_by(VecOrInt::compare);

            let mut key = 1 + packets.iter().position(|x| *x == divider1).unwrap();
            key *= 1 + packets.iter().position(|x| *x == divider2).unwrap();
            key.to_string()
        }
    }

    fn run_preliminary_tasks(&mut self, input: &'a str) {
        self.pairs = input
            .split("\n\n")
            .map(|l| {
                let (a_str, b_str) = l.split_once('\n').unwrap();
                (a_str.parse().unwrap(), b_str.parse().unwrap())
            })
            .collect();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum VecOrInt {
    V(Vec<VecOrInt>),
    I(u32),
}

impl VecOrInt {
    pub fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Self> {
        match chars.next() {
            Some(c) if c.is_digit(10) => {
                let val = match chars.peek() {
                    // double digit value edge case
                    Some('0') => {
                        chars.next();
                        10
                    }
                    _ => c as u32 - '0' as u32,
                };
                Some(VecOrInt::I(val))
            }
            Some('[') => {
                let mut ret_vec = Vec::new();

                // recursively process in an iterative manner
                while let Some(val) = Self::parse(chars) {
                    ret_vec.push(val);
                    if chars.next() == Some(']') {
                        break;
                    }
                }

                Some(VecOrInt::V(ret_vec))
            }
            _ => None,
        }
    }

    pub fn compare(a: &Self, b: &Self) -> Ordering {
        match (a, b) {
            (VecOrInt::V(a), VecOrInt::V(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    let ordering = VecOrInt::compare(a, b);
                    if ordering.is_ne() {
                        return ordering;
                    }
                }
                a.len().cmp(&b.len())
            }
            (VecOrInt::I(a), VecOrInt::I(b)) => a.cmp(b),
            (VecOrInt::V(_), VecOrInt::I(_)) => VecOrInt::compare(a, &VecOrInt::V(vec![b.clone()])),
            (VecOrInt::I(_), VecOrInt::V(_)) => VecOrInt::compare(&VecOrInt::V(vec![a.clone()]), b),
        }
    }
}

impl FromStr for VecOrInt {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(VecOrInt::parse(&mut s.chars().peekable()).unwrap())
    }
}
