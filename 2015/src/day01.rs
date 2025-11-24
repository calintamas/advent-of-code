// --- Day 1: Not Quite Lisp ---

pub type Input = Vec<char>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .collect()
}

pub fn part1(input: &Input) -> String {
    input
        .iter()
        .fold(0, |floor, ch| execute_instruction(floor, ch))
        .to_string()
}

pub fn part2(input: &Input) -> String {
    input
        .iter()
        .enumerate()
        .try_fold(0, |floor, (index, ch)| {
            let new_floor = execute_instruction(floor, ch);
            if new_floor == -1 {
                Err(index + 1)
            } else {
                Ok(new_floor)
            }
        })
        .unwrap_err()
        .to_string()
}

fn execute_instruction(current_floor: i32, instruction: &char) -> i32 {
    current_floor
        + match instruction {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
}
