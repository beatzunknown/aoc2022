use aoc2022_lib::{Day, Part};
use std::collections::HashSet;
use std::ops::AddAssign;
use text_io::scan;

#[derive(Default, Eq, Hash, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn neighbouring(&self, target: Self) -> bool {
        (-1..=1).contains(&(self.0 - target.0)) && (-1..=1).contains(&(self.1 - target.1))
    }

    fn follow(&mut self, target: Self) {
        if self.neighbouring(target) {
            return;
        }

        let dx = (self.0 != target.0) as i32 * (-1_i32).pow((self.0 > target.0) as u32);
        let dy = (self.1 != target.1) as i32 * (-1_i32).pow((self.1 > target.1) as u32);

        *self += Pos(dx, dy);
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

fn get_dir(dir: char) -> Pos {
    match dir {
        'L' => Pos(-1, 0),
        'R' => Pos(1, 0),
        'U' => Pos(0, 1),
        'D' => Pos(0, -1),
        _ => Pos::default(),
    }
}

#[derive(Default)]
pub struct Day09 {}

impl<'a> Day<'a> for Day09 {
    fn solve(&mut self, input: &'a str, part: Part) -> String {
        let mut visited: HashSet<Pos> = HashSet::from([Pos::default()]);
        let mut knots: Vec<Pos> = match part {
            Part::One => vec![Pos::default(); 2],
            Part::Two => vec![Pos::default(); 10],
        };

        for l in input.lines() {
            let (d, n): (char, usize);
            scan!(l.bytes() => "{} {}", d, n);
            for _ in 0..n {
                knots[0] += get_dir(d);
                for i in 1..knots.len() {
                    let prev = knots[i - 1];
                    knots[i].follow(prev);
                }
                visited.insert(*knots.last().unwrap());
            }
        }
        visited.len().to_string()
    }
}
