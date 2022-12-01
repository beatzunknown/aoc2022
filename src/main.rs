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
