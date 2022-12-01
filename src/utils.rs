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
