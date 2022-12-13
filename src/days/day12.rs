use aoc2022_lib::{Day, Part};
use pathfinding::prelude::dijkstra;
use std::ops::Add;

#[derive(Default)]
pub struct Day12 {
    starts: Vec<Pos>,
    end: Pos,
    grid: Vec<Vec<u32>>,
}

impl<'a> Day<'a> for Day12 {
    fn solve(&mut self, _input: &'a str, part: Part) -> String {
        if part.is_first() {
            dijkstra(
                self.starts.last().unwrap(),
                |p| p.get_neighbours(&self.grid),
                |p| *p == self.end,
            )
            .unwrap()
            .1
            .to_string()
        } else {
            self.starts
                .iter()
                .map(|start| {
                    dijkstra(start, |p| p.get_neighbours(&self.grid), |p| *p == self.end)
                        .unwrap_or((Vec::new(), 10000000))
                        .1
                })
                .min()
                .unwrap()
                .to_string()
        }
    }

    fn run_preliminary_tasks(&mut self, input: &'a str) {
        let mut start = Pos(0, 0);

        for (i, line) in input.lines().enumerate() {
            let mut row: Vec<u32> = Vec::new();
            for (j, c) in line.chars().enumerate() {
                row.push(match c {
                    'S' => {
                        start = Pos(i as i32, j as i32);
                        0
                    }
                    'E' => {
                        self.end = Pos(i as i32, j as i32);
                        25
                    }
                    'a' => {
                        self.starts.push(Pos(i as i32, j as i32));
                        0
                    }
                    x => x as u32 - 'a' as u32,
                });
            }
            self.grid.push(row);
        }
        self.starts.push(start);
    }
}

#[derive(Default, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Pos(i32, i32);

impl Pos {
    pub fn get_neighbours(&self, grid: &Vec<Vec<u32>>) -> Vec<(Pos, usize)> {
        let n_rows = grid.len() as i32;
        let n_cols = grid[0].len() as i32;
        let mut neighbours: Vec<Pos> = Vec::new();

        for dir in [Pos(-1, 0), Pos(1, 0), Pos(0, 1), Pos(0, -1)] {
            let new = *self + dir;
            if (0..n_rows).contains(&new.0)
                && (0..n_cols).contains(&new.1)
                && grid[new.0 as usize][new.1 as usize]
                    .checked_sub(grid[self.0 as usize][self.1 as usize])
                    <= Some(1)
            {
                neighbours.push(new);
            }
        }

        neighbours.into_iter().map(|p| (p, 1)).collect()
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}
