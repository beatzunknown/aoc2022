use aoc2022_lib::{utils, Day, Part};
use std::cmp;
use text_io::scan;

#[derive(Default)]
pub struct Day08 {}

impl<'a> Day<'a> for Day08 {
    fn solve(&mut self, input: &'a str, part: Part) -> String {
        let trees: Vec<Vec<u32>> = input
            .lines()
            .map(|l| l.trim().chars().filter_map(|x| x.to_digit(10)).collect())
            .collect();
        let n_rows = trees.len();
        let n_cols = trees[0].len();
        let mut n_visible = 2 * n_rows + 2 * n_cols - 4;
        let mut max_scenic = 0;
        for i in 1..n_rows - 1 {
            for j in 1..n_cols - 1 {
                let x = trees[i][j];
                let mut visible_sides = 4;
                let mut scores: Vec<usize> = Vec::with_capacity(4);
                for k in (0..i).rev() {
                    if x <= trees[k][j] {
                        visible_sides -= 1;
                        scores.push(i - k);
                        break;
                    }
                }
                if scores.len() == 0 {
                    scores.push(i);
                }
                for k in i + 1..n_rows {
                    if x <= trees[k][j] {
                        visible_sides -= 1;
                        scores.push(k - i);
                        break;
                    }
                }
                if scores.len() == 1 {
                    scores.push(n_rows - i - 1);
                }
                for k in (0..j).rev() {
                    if x <= trees[i][k] {
                        visible_sides -= 1;
                        scores.push(j - k);
                        break;
                    }
                }
                if scores.len() == 2 {
                    scores.push(j)
                }
                for k in j + 1..n_cols {
                    if x <= trees[i][k] {
                        visible_sides -= 1;
                        scores.push(k - j);
                        break;
                    }
                }
                if scores.len() == 3 {
                    scores.push(n_cols - j - 1);
                }
                if visible_sides > 0 {
                    n_visible += 1;
                }
                max_scenic = cmp::max(max_scenic, scores.iter().product());
            }
        }
        if part.is_first() {
            n_visible.to_string()
        } else {
            max_scenic.to_string()
        }
    }
}
