use crate::advent_day::AdventDay;

pub struct Day09 {
    data: Vec<String>,
}

impl Day09 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl AdventDay for Day09 {
    fn parse(&mut self, input: &str) {
        self.data = expand(input.trim());
    }

    fn p1(&self) -> String {
        let mut cpy = self.data.clone();

        let mut start = cpy.iter().position(|x| x == ".").unwrap();

        for i in (0..cpy.len()).rev() {
            if start >= i {
                break;
            }
            if cpy[i] != "." {
                cpy[start] = cpy[i].clone();
                cpy[i] = ".".to_string();
                start = cpy.iter().position(|x| x == ".").unwrap();
            }
        }

        let sum = cpy.iter().enumerate().fold(0, |mut acc, (idx, file)| {
            if file != "." {
                let val = file.parse::<usize>().unwrap();
                acc += val * idx;
            }
            acc
        });

        sum.to_string()
    }

    fn p2(&self) -> String {
        "".to_string()
    }
}

fn expand(input: &str) -> Vec<String> {
    let mut id = 0;
    let mut output: Vec<String> = Vec::new();

    for (idx, ch) in input.chars().enumerate() {
        let digit = ch.to_digit(10).unwrap() as usize;
        if digit <= 0 {
            continue;
        }
        if idx % 2 == 0 {
            for _ in 0..digit {
                output.push(id.to_string());
            }
            id += 1;
        } else {
            for _ in 0..digit {
                output.push('.'.to_string());
            }
        }
    }

    output
}

#[test]
fn test_expand() {
    let input = "233313312141413140224";
    assert_eq!(
        expand(input),
        vec![
            "0", "0", ".", ".", ".", "1", "1", "1", ".", ".", ".", "2", ".", ".", ".", "3", "3",
            "3", ".", "4", "4", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".", "7", "7",
            "7", ".", "8", "8", "8", "8", "9", "9", ".", ".", "10", "10", "10", "10"
        ] // "00...111...2...333.44.5555.6666.777.888899..10101010"
    )
}

#[test]
fn test_p1() {
    let input = "2333133121414131402";
    let mut day = Day09::new();
    day.parse(input);
    assert_eq!(day.p1(), "1928");
}
