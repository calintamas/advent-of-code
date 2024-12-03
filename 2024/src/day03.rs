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

enum Instruction {
    Mul(i32, i32), // mul(X, Y)
    Do,            // do()
    Dont,          // don't()
}

fn parse(input: &String, re: &str) -> Vec<Instruction> {
    let regex = Regex::new(re).unwrap();
    regex
        .captures_iter(input)
        .filter_map(|item| {
            if let Some(x_match) = item.get(1) {
                // match mul(X,Y)
                let x = x_match.as_str().parse::<i32>().unwrap();
                let y = item.get(2).unwrap().as_str().parse::<i32>().unwrap();
                Some(Instruction::Mul(x, y))
            } else if let Some(instr) = item.get(0) {
                // match "do()" or "don't()"
                match instr.as_str() {
                    "do()" => Some(Instruction::Do),
                    "don't()" => Some(Instruction::Dont),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
}

fn run_p1(input: &String) -> i32 {
    let re = r"mul\((\d{1,3}),(\d{1,3})\)";
    parse(input, re).iter().fold(0, |mut acc, instr| {
        match instr {
            Instruction::Mul(x, y) => acc += x.mul(y),
            _ => {}
        }
        acc
    })
}

fn run_p2(input: &String) -> i32 {
    let re = r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)";
    let mut should_do = true;

    parse(input, re).iter().fold(0, |mut acc, instr| {
        match instr {
            Instruction::Mul(x, y) => {
                if should_do {
                    acc += x.mul(y);
                }
            }
            Instruction::Do => should_do = true,
            Instruction::Dont => should_do = false,
        }
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
