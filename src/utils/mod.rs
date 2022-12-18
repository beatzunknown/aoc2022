use num::Num;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

// thanks Fornwall
// https://github.com/fornwall/advent-of-code/blob/master/crates/core/src/common/parser.rs
pub fn parse_lines<T: FromStr>(input: &str) -> Result<Vec<T>, String> {
    input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            line.parse::<T>()
                .map_err(|_| format!("Line {}: Not a valid integer", line_idx + 1))
        })
        .collect()
}

pub fn intersect_sets<T, I>(sets: I) -> Option<HashSet<T>>
where
    I: IntoIterator<Item = HashSet<T>>,
    T: Eq + Hash + Copy,
{
    let mut iter = sets.into_iter();
    iter.next().map(|set| {
        iter.fold(set, |a, b| {
            a.intersection(&b).copied().collect::<HashSet<T>>()
        })
    })
}

pub fn split_to_vec<T: FromStr>(input: &str, delim: &str) -> Result<Vec<T>, String> {
    input
        .split(delim)
        .enumerate()
        .map(|(i, x)| {
            x.parse::<T>()
                .map_err(|_| format!("Element {} malformed", i + 1))
        })
        .collect()
}

#[derive(Default, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Pos<T> {
    x: T,
    y: T,
}

impl<T> Pos<T>
where
    T: Num + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    // todo: fix to work for generic T
    // pub fn neighbouring(&self, target: Self) -> bool {
    //     (-1..=1).contains(&(self.x - target.x)) && (-1..=1).contains(&(self.y - target.y))
    // }

    // pub fn follow(&mut self, target: Self) {
    //     if self.neighbouring(target) {
    //         return;
    //     }
    //
    //     let dx = (self.x != target.0) as i32 * (-1_i32).pow((self.x > target.0) as u32);
    //     let dy = (self.y != target.1) as i32 * (-1_i32).pow((self.y > target.1) as u32);
    //
    //     *self += Pos::new(dx, dy);
    // }

    // need to sort out neighbour direction generic for type R
    // pub fn get_neighbours_for_dijkstra(&self, grid: &Vec<Vec<u32>>) -> Vec<(Pos<T>, usize)> {
    //     let n_rows = grid.len() as i32;
    //     let n_cols = grid[0].len() as i32;
    //     let mut neighbours: Vec<Pos<T>> = Vec::new();
    //
    //     for dir in [
    //         Pos::new(-1, 0),
    //         Pos::new(1, 0),
    //         Pos::new(0, 1),
    //         Pos::new(0, -1),
    //     ] {
    //         let new = *self + dir;
    //         if (0..n_rows).contains(&new.0)
    //             && (0..n_cols).contains(&new.1)
    //             && grid[new.0 as usize][new.1 as usize]
    //                 .checked_sub(grid[self.x as usize][self.y as usize])
    //                 <= Some(1)
    //         {
    //             neighbours.push(new);
    //         }
    //     }
    //
    //     neighbours.into_iter().map(|p| (p, 1)).collect()
    // }
}

// can't genericise this due to unstable iter::Step trait
impl Pos<i32> {
    pub fn interval_range_to(&self, point: &Pos<i32>) -> Vec<Pos<i32>> {
        if self.x == point.x {
            if self.y < point.y {
                (self.y..=point.y).map(|y| Pos::new(self.x, y)).collect()
            } else {
                (point.y..=self.y).map(|y| Pos::new(self.x, y)).collect()
            }
        } else if self.y == point.y {
            if self.x < point.x {
                (self.x..=point.x).map(|x| Pos::new(x, self.y)).collect()
            } else {
                (point.x..=self.x).map(|x| Pos::new(x, self.y)).collect()
            }
        } else {
            Vec::new()
        }
    }
}

impl<T: Num + Copy> Add<Pos<T>> for Pos<T> {
    type Output = Pos<T>;

    fn add(self, rhs: Pos<T>) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Num + Copy + AddAssign> AddAssign<Pos<T>> for Pos<T> {
    fn add_assign(&mut self, rhs: Pos<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Num + Copy + FromStr> FromStr for Pos<T> {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| AoCError::ParseError("Can't split position".to_string()))?;
        Ok(Pos::new(
            x.parse()
                .map_err(|_| AoCError::ParseError("Can't parse x coord".to_string()))?,
            y.parse()
                .map_err(|_| AoCError::ParseError("Can't parse y coord".to_string()))?,
        ))
    }
}

#[derive(Debug)]
pub enum AoCError {
    ParseError(String),
}
