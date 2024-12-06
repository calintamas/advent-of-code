use advent_day::AdventDay;
use std::env;
use std::fs::read_to_string;
use std::str::FromStr;

mod advent_day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod grid;

fn main() {
    let mut args = env::args().skip(1);

    let input_file_path = args.next().unwrap();
    let day_str = args.next().unwrap();

    let day = u32::from_str(&day_str).unwrap();
    let input = read_to_string(&input_file_path).unwrap();

    run_day(day, &input);
}

fn run_day(day: u32, input: &str) {
    let mut advent_day: Box<dyn AdventDay> = match day {
        1 => Box::new(day01::Day01::new()),
        2 => Box::new(day02::Day02::new()),
        3 => Box::new(day03::Day03::new()),
        4 => Box::new(day04::Day04::new()),
        5 => Box::new(day05::Day05::new()),
        6 => Box::new(day06::Day06::new()),
        _ => {
            println!("Day not implemented");
            return;
        }
    };
    advent_day.run(input);
}
