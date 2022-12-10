use aoc2022_lib::{Day, Part};
use text_io::scan;

#[derive(Default)]
pub struct Day10 {
    cycle_hist: Vec<i32>,
}

impl<'a> Day<'a> for Day10 {
    fn solve(&mut self, _input: &'a str, part: Part) -> String {
        if part.is_first() {
            (20..=220)
                .step_by(40)
                .map(|i| i as i32 * self.cycle_hist[i - 1])
                .sum::<i32>()
                .to_string()
        } else {
            let mut crt = String::from("\n");
            for r in 0..6_i32 {
                for c in 0..40_i32 {
                    let x = self.cycle_hist[(r * 40 + c) as usize];
                    crt += match (x - 1..=x + 1).contains(&(c)) {
                        true => "#",
                        false => ".",
                    };
                }
                crt += "\n";
            }
            crt
        }
    }

    fn run_preliminary_tasks(&mut self, input: &'a str) {
        let mut x: i32 = 1;
        self.cycle_hist.push(x);
        for l in input.lines() {
            self.cycle_hist.push(x);
            if l != "noop" {
                let inc: i32;
                scan!(l.bytes() => "addx {}", inc);
                x += inc;
                self.cycle_hist.push(x);
            }
        }
    }
}
