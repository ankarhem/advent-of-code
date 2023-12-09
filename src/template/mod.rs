use crate::{Day, Year};
use std::{env, fs};

pub mod aoc_cli;
pub mod commands;
pub mod readme_benchmarks;
pub mod runner;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
#[must_use]
pub fn read_file(folder: &str, year: Year, day: Day) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{year}_{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Creates the constants `YEAR`, `DAY` and sets up the input and runner for each part.
#[macro_export]
macro_rules! solution {
    ($year:expr, $day:expr) => {
        /// The current day.
        const YEAR: advent_of_code::Year = advent_of_code::year!($year);
        const DAY: advent_of_code::Day = advent_of_code::day!($day);

        fn main() {
            use advent_of_code::template::runner::*;
            let input = advent_of_code::template::read_file("inputs", YEAR, DAY);
            run_part(part_one, &input, YEAR, DAY, 1);
            run_part(part_two, &input, YEAR, DAY, 2);
        }
    };
}
