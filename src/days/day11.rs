use aoc2022_lib::{Day, Part};
use num::Integer;
use regex::Regex;
use std::io::Error;
use std::str::FromStr;
use structopt::lazy_static::lazy_static;

#[derive(Default)]
pub struct Day11 {}

impl<'a> Day<'a> for Day11 {
    fn solve(&mut self, input: &'a str, part: Part) -> String {
        let mut monkeys: Vec<Monkey> = input
            .split("\n\n")
            .map(|l| l.parse::<Monkey>().unwrap())
            .collect();

        if part.is_first() {
            for _ in 0..20 {
                for i in 0..monkeys.len() {
                    let monkey = &mut monkeys[i];
                    monkey
                        .drain_items(divide_three)
                        .iter()
                        .for_each(|&(item, monkey)| monkeys[monkey as usize].add_item(item));
                }
            }
        } else {
            let lcm = monkeys
                .iter()
                .map(|monkey| monkey.divisor)
                .fold(1, |acc, d| acc.lcm(&d));

            for _ in 0..10000 {
                for i in 0..monkeys.len() {
                    let monkey = &mut monkeys[i];
                    monkey
                        .drain_items(|item| lcm_wrap(item, lcm))
                        .iter()
                        .for_each(|&(item, monkey)| monkeys[monkey as usize].add_item(item));
                }
            }
        }

        let mut counts: Vec<usize> = monkeys.iter().map(|m| m.inspections as usize).collect();
        counts.sort();
        counts.reverse();

        counts.iter().take(2).product::<usize>().to_string()
    }
}

fn divide_three(dividend: usize) -> usize {
    (dividend as f32 / 3_f32).floor() as usize
}

fn lcm_wrap(dividend: usize, lcm: usize) -> usize {
    dividend.rem_euclid(lcm)
}

struct Monkey {
    inspections: usize,
    items: Vec<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    divisor: usize,
    throw_to_true: usize,
    throw_to_false: usize,
}

impl Monkey {
    pub fn drain_items(&mut self, minimiser: impl Fn(usize) -> usize) -> Vec<(usize, usize)> {
        self.inspections += self.items.len();
        self.items
            .drain(..)
            .map(|mut x| {
                x = minimiser((self.op)(x));
                let dest = if x.rem_euclid(self.divisor) == 0 {
                    self.throw_to_true
                } else {
                    self.throw_to_false
                };
                (x, dest)
            })
            .collect()
    }

    pub fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }

        let mut iter = s.lines();
        let items = RE
            .find_iter(iter.nth(1).unwrap())
            .map(|i| i.as_str().parse().unwrap())
            .collect();
        let mut vals = iter.map(|l| match RE.captures(l) {
            Some(c) => c.get(0).unwrap().as_str().parse::<usize>().ok(),
            None => None,
        });

        let op_num: Option<usize> = vals.next().unwrap();
        let op: Box<dyn Fn(usize) -> usize> = match s.contains("+") {
            true => match op_num {
                Some(num) => Box::new(move |x| x + num),
                None => Box::new(|x| x + x),
            },
            false => match op_num {
                Some(num) => Box::new(move |x| x * num),
                None => Box::new(|x| x * x),
            },
        };

        Ok(Monkey {
            inspections: 0,
            items,
            op,
            divisor: vals.next().unwrap().unwrap(),
            throw_to_true: vals.next().unwrap().unwrap(),
            throw_to_false: vals.next().unwrap().unwrap(),
        })
    }
}
