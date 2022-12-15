use std::collections::HashSet;
use std::hash::Hash;
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
