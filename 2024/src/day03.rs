use std::ops::Mul;

use crate::advent_day::AdventDay;
use regex::Regex;

pub struct Day03 {
    input: String,
}

impl Day03 {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
}

impl AdventDay for Day03 {
    fn parse(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn p1(&self) -> String {
        run_p1(&self.input).to_string()
    }

    fn p2(&self) -> String {
        run_p2(&self.input).to_string()
    }
}

fn extract_p1(input: &String) -> Vec<(i32, i32)> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex
        .captures_iter(input)
        .map(|item| {
            let x = item.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = item.get(2).unwrap().as_str().parse::<i32>().unwrap();
            return (x, y);
        })
        .collect()
}

fn run_p1(input: &String) -> i32 {
    let pairs_to_multiply = extract_p1(input);
    pairs_to_multiply.iter().fold(0, |mut acc, pair| {
        acc += pair.0.mul(pair.1);
        acc
    })
}

fn extract_p2(input: &String) -> Vec<Result<(i32, i32), String>> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    regex
        .captures_iter(input)
        .map(|item| {
            // match mul(X,Y)
            if let Some(x_match) = item.get(1) {
                let x = x_match.as_str().parse::<i32>().unwrap();
                let y = item.get(2).unwrap().as_str().parse::<i32>().unwrap();
                Ok((x, y))
            }
            // match "do()" or "don't()"
            else if let Some(do_match) = item.get(0) {
                Err(do_match.as_str().to_string())
            } else {
                Err("Unknown match".to_string())
            }
        })
        .collect()
}

fn run_p2(input: &String) -> i32 {
    let instructions = extract_p2(input);
    let mut should_do = true;
    let mut sum: i32 = 0;

    for item in instructions {
        match item {
            Ok((x, y)) => {
                if should_do {
                    sum += x.mul(y);
                }
            }
            Err(instr) => {
                if instr == "do()" {
                    should_do = true
                }
                if instr == "don't()" {
                    should_do = false
                }
            }
        }
    }
    sum
}

#[test]
fn test_run_p1() {
    assert_eq!(
        run_p1(
            &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()
        ),
        161
    )
}

#[test]
fn test_run_p2() {
    assert_eq!(
        run_p2(
            &"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                .to_string()
        ),
        48
    )
}
