use crate::advent_day::AdventDay;

pub struct Day01 {
    instructions: Vec<char>,
}

impl Day01 {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn execute_instruction(current_floor: i32, instruction: &char) -> i32 {
        current_floor
            + match instruction {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
    }
}

impl AdventDay for Day01 {
    fn parse(&mut self, input: &str) {
        self.instructions.extend(
            input
                .lines()
                .filter(|line| !line.is_empty())
                .flat_map(|line| line.chars()),
        );
    }

    fn p1(&self) -> String {
        self.instructions
            .iter()
            .fold(0, |floor, ch| Self::execute_instruction(floor, ch))
            .to_string()
    }

    fn p2(&self) -> String {
        self.instructions
            .iter()
            .enumerate()
            .try_fold(0, |floor, (index, ch)| {
                let new_floor = Self::execute_instruction(floor, ch);
                if new_floor == -1 {
                    Err(index + 1)
                } else {
                    Ok(new_floor)
                }
            })
            .unwrap_err()
            .to_string()
    }
}
