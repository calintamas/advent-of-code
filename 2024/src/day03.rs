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
        "".to_string()
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

#[test]
fn test_run_p1() {
    assert_eq!(
        run_p1(
            &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()
        ),
        161
    )
}
