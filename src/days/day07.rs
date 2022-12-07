use aoc2022_lib::{Day, Part};

#[derive(Default)]
pub struct Day07 {
    sizes: Vec<u32>,
}

impl<'a> Day<'a> for Day07 {
    fn solve(&mut self, _input: &'a str, part: Part) -> String {
        if part.is_first() {
            return self
                .sizes
                .iter()
                .filter(|x| **x <= 100000)
                .sum::<u32>()
                .to_string();
        }

        let space_required = 30000000 - (70000000 - self.sizes.last().unwrap());
        self.sizes
            .iter()
            .find(|x| **x >= space_required)
            .unwrap()
            .to_string()
    }

    fn run_preliminary_tasks(&mut self, input: &'a str) {
        let lines: Vec<&str> = input.trim().lines().skip(1).collect();
        let _ = calc_sizes(&lines, 0, &mut self.sizes);
        self.sizes.sort();
    }
}

fn calc_sizes(lines: &Vec<&str>, mut index: usize, sizes: &mut Vec<u32>) -> (usize, u32) {
    let mut total: u32 = 0;
    let n = lines.len();
    while index < n {
        let line = lines[index];
        index += 1;
        if line.eq("$ ls") || line.starts_with("dir") {
            continue;
        }

        let mut split = line.split_ascii_whitespace();
        let first = split.next().unwrap();
        if !first.eq("$") {
            total += first.parse::<u32>().unwrap();
            continue;
        }

        let dir = split.nth(1).unwrap();
        if dir.eq("..") {
            break;
        }

        let (new_index, sub_total) = calc_sizes(lines, index, sizes);
        index = new_index;
        total += sub_total;
    }
    sizes.push(total);
    (index, total)
}
