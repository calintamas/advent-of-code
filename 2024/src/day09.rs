use crate::advent_day::AdventDay;

pub struct Day09 {
    input: String,
}

impl Day09 {
    pub fn new() -> Self {
        Self {
            input: "".to_string(),
        }
    }
}

impl AdventDay for Day09 {
    fn parse(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn p1(&self) -> String {
        let mut cpy = expand(self.input.trim());

        let mut start = cpy.iter().position(|x| x == ".").unwrap();

        for i in (0..cpy.len()).rev() {
            if start >= i {
                break;
            }
            if cpy[i] != "." {
                cpy.swap(i, start);
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
        let mut blocks = expand2(self.input.trim());

        for idx in (0..blocks.len()).rev() {
            let block = &blocks[idx];
            let block_size = block.len();
            // Find an empty block with this size
            if let Some(front_idx) = blocks.iter().position(|x| {
                let empty_block_size = x.iter().filter(|&&x| x == Item::None).count();
                empty_block_size >= block_size
            }) {
                if front_idx > idx {
                    continue;
                }

                let mut new_front_block: Block = Vec::new();
                let front_block = &blocks[front_idx];

                let mut i = 0;
                for file in front_block.iter() {
                    match file {
                        Item::File(_) => new_front_block.push(*file),
                        Item::None => {
                            if i < block.len() {
                                new_front_block.push(block[i]);
                                i += 1;
                            } else {
                                new_front_block.push(Item::None);
                            }
                        }
                    }
                }

                blocks[front_idx] = new_front_block;
                blocks[idx] = (0..block_size).map(|_| Item::None).collect();
            }
        }

        let mut i = 0;
        let sum = blocks.iter().fold(0, |mut acc, block| {
            for file in block {
                match file {
                    Item::File(val) => acc += val * i,
                    Item::None => {}
                }
                i += 1;
            }

            acc
        });

        sum.to_string()
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Item {
    File(usize),
    None,
}
type Block = Vec<Item>;

fn expand2(input: &str) -> Vec<Block> {
    let mut id = 0;
    let mut output: Vec<Block> = Vec::new();

    for (idx, ch) in input.chars().enumerate() {
        let size = ch.to_digit(10).unwrap() as usize;
        if size <= 0 {
            continue;
        }
        if idx % 2 == 0 {
            let values = (0..size).map(|_| Item::File(id)).collect();
            output.push(values);
            id += 1;
        } else {
            let values = (0..size).map(|_| Item::None).collect();
            output.push(values);
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

#[test]
fn test_p2() {
    let input = "2333133121414131402";
    let mut day = Day09::new();
    day.parse(input);
    assert_eq!(day.p2(), "2858");
}
