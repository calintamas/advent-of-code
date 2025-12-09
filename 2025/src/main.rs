use std::env;
use std::fs::read_to_string;
use std::str::FromStr;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let mut args = env::args().skip(1);

    let day_str = args.next().unwrap();
    let day = u32::from_str(&day_str).unwrap();

    let input_file_path = format!("./input/day{:02}.txt", day);
    let input = read_to_string(&input_file_path).unwrap();

    run_day(day, &input);
}

macro_rules! run_solution {
    ($module:ident, $input:expr) => {{
        let parsed = $module::parse($input);
        println!("Part 1: {}", $module::part1(&parsed));
        println!("Part 2: {}", $module::part2(&parsed));
    }};
}

fn run_day(day: u32, input: &str) {
    match day {
        1 => run_solution!(day01, input),
        2 => run_solution!(day02, input),
        3 => run_solution!(day03, input),
        4 => run_solution!(day04, input),
        5 => run_solution!(day05, input),
        6 => run_solution!(day06, input),
        7 => run_solution!(day07, input),
        _ => println!("Day not implemented"),
    }
}
