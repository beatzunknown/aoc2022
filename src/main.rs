use std::error::Error;
use std::fs;
use std::process;

use anyhow::Context;
use structopt::StructOpt;

use aoc2022_lib::{Day, INPUT_PREFIX, INPUT_SUFFIX};
use days::*;

mod days;

#[derive(Debug, StructOpt)]
#[structopt(name = "AoC2022", about = "Advent of Code 2022 Solver")]
struct Opt {
    #[structopt(name = "DAY")]
    day: u8,
}

// todo: proper error handling
fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    if opt.day == 0 || opt.day > 25 {
        println!("Day number must be between 1-25 inclusive.");
        process::exit(1);
    }

    let input_file = format!("{}{:0>2}{}", INPUT_PREFIX, opt.day, INPUT_SUFFIX);

    let input = fs::read_to_string(input_file.clone())
        .context(format!("Unable to open input file '{}'", input_file))?;

    let mut day_to_solve: Box<dyn Day> = match opt.day {
        1 => Box::new(day01::Day01::default()),
        2 => Box::new(day02::Day02::default()),
        3 => Box::new(day03::Day03::default()),
        4 => Box::new(day04::Day04::default()),
        5 => Box::new(day05::Day05::default()),
        6 => Box::new(day06::Day06::default()),
        7 => Box::new(day07::Day07::default()),
        8 => Box::new(day08::Day08::default()),
        9 => Box::new(day09::Day09::default()),
        10 => Box::new(day10::Day10::default()),
        11 => Box::new(day11::Day11::default()),
        12 => Box::new(day12::Day12::default()),
        13 => Box::new(day13::Day13::default()),
        14 => Box::new(day14::Day14::default()),
        15 => Box::new(day15::Day15::default()),
        _ => {
            println!(
                "This is an invalid or not yet reached day number: '{}'",
                opt.day
            );
            // yes this is bad but it'll do for now
            process::exit(1);
        }
    };

    let solution = day_to_solve.get_solutions(&input);

    println!("{solution}");

    Ok(())
}
