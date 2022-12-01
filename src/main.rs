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
    day: i8,
}

// todo: proper error handling
fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let day_to_solve = match opt.day {
        1 => day01::Day01 {},
        _ => {
            println!(
                "This is an invalid or not yet reached day number: '{}'",
                opt.day
            );
            // yes this is bad but it'll do for now
            process::exit(1);
        }
    };

    let input_file = format!("{}{:0>2}{}", INPUT_PREFIX, opt.day, INPUT_SUFFIX);

    let input = fs::read_to_string(input_file.clone())
        .context(format!("Unable to open input file '{}'", input_file))?;

    let solution = day_to_solve.get_solutions(&input);

    println!("{solution}");

    Ok(())
}
