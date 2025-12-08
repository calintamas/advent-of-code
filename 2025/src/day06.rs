// --- Day 6: Trash Compactor ---

pub struct Input {
    pub ops: Vec<char>,       // row with operations (* or +)
    pub rows: Vec<Vec<char>>, // rows with values
}
pub fn parse(input: &str) -> Input {
    let rows: Vec<&str> = input.lines().collect();
    Input {
        ops: rows.last().unwrap().chars().collect(),
        rows: rows[..rows.len() - 1]
            .iter()
            .map(|line| line.chars().collect())
            .collect(),
    }
}

pub fn part1(input: &Input) -> String {
    let ops = &input.ops;
    let rows = &input.rows;

    let mut total = 0;

    let mut col_idx = 0;
    let mut op_idx = 0;

    while col_idx < ops.len() {
        let mut op: Option<char> = None;

        for idx in col_idx..ops.len() {
            let ch = ops[idx];
            if ch != ' ' {
                if op.is_none() {
                    op = Some(ch);
                    op_idx = idx;
                } else {
                    break;
                }
            }
            col_idx = idx;
        }
        // always go to prev column with numeric values, except for last column
        if col_idx != ops.len() - 1 {
            col_idx -= 1
        }

        let mut result: usize = match op {
            Some('*') => 1,
            Some('+') => 0,
            _ => panic!("unexpected operation: {:?}", op),
        };

        for chars in rows.iter() {
            let mut value = String::new();
            for idx in op_idx..=col_idx {
                let ch = chars[idx];
                if ch != ' ' {
                    value.push(ch);
                }
            }
            result = apply_operation(op, result, value.parse().unwrap());
        }

        total += result;
        col_idx += 2; // jump over empty column
    }

    total.to_string()
}

pub fn part2(input: &Input) -> String {
    let ops = &input.ops;
    let rows = &input.rows;

    let mut total = 0;

    let mut col_idx = 0;
    let mut op_idx = 0;

    while col_idx < ops.len() {
        let mut op: Option<char> = None;

        for idx in col_idx..ops.len() {
            let ch = ops[idx];
            if ch != ' ' {
                if op.is_none() {
                    op = Some(ch);
                    op_idx = idx;
                } else {
                    break;
                }
            }
            col_idx = idx;
        }
        if col_idx != ops.len() - 1 {
            col_idx -= 1
        }

        let mut result: usize = match op {
            Some('*') => 1,
            Some('+') => 0,
            _ => panic!("unexpected operation: {:?}", op),
        };

        let mut col = col_idx;
        loop {
            let mut value = String::new();
            for r in 0..rows.len() {
                let ch = rows[r][col];
                if ch != ' ' {
                    value.push(ch);
                }
            }
            result = apply_operation(op, result, value.parse().unwrap());
            if col == op_idx {
                break;
            }
            col -= 1;
        }

        total += result;
        col_idx += 2; // jump over empty column
    }

    total.to_string()
}

fn apply_operation(op: Option<char>, acc: usize, value: usize) -> usize {
    match op {
        Some('*') => acc * value,
        Some('+') => acc + value,
        _ => panic!("unexpected operation: {:?}", op),
    }
}
