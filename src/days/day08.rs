use aoc2022_lib::{Day, Part};
use std::cmp;

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
                let mut visible = false;
                let mut score = 1;
                score *= match (0..i).rev().find(|&k| x <= trees[k][j]) {
                    None => {
                        visible = true;
                        i
                    }
                    Some(h) => i - h,
                };
                score *= match (i + 1..n_rows).find(|&k| x <= trees[k][j]) {
                    None => {
                        visible = true;
                        n_rows - i - 1
                    }
                    Some(h) => h - i,
                };
                score *= match (0..j).rev().find(|&k| x <= trees[i][k]) {
                    None => {
                        visible = true;
                        j
                    }
                    Some(h) => j - h,
                };
                score *= match (j + 1..n_cols).find(|&k| x <= trees[i][k]) {
                    None => {
                        visible = true;
                        n_rows - j - 1
                    }
                    Some(h) => h - j,
                };

                n_visible += visible as usize;
                max_scenic = cmp::max(max_scenic, score);
            }
        }

        if part.is_first() {
            n_visible.to_string()
        } else {
            max_scenic.to_string()
        }
    }
}
