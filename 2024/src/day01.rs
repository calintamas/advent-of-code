use crate::advent_day::AdventDay;

pub struct Day01 {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl Day01 {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}

impl AdventDay for Day01 {
    fn parse(&mut self, input: &str) {
        for line in input.lines() {
            let values: Vec<u64> = line
                .split("   ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            self.left.push(values[0]);
            self.right.push(values[1]);
        }
    }

    fn p1(&self) -> String {
        let sum = total_distance(self.left.clone(), self.right.clone());
        return sum.to_string();
    }

    fn p2(&self) -> String {
        " ".to_string()
    }
}

fn total_distance(mut left: Vec<u64>, mut right: Vec<u64>) -> i64 {
    left.sort(); // sort in place
    right.sort();

    let mut diffs: Vec<i64> = Vec::new();

    for (index, left_item) in left.iter().enumerate() {
        if let Some(right_item) = right.get(index) {
            diffs.push(*left_item as i64 - *right_item as i64);
        } else {
            eprintln!("Index {} out of bounds", index);
        }
    }

    diffs.iter_mut().for_each(|x| *x = x.abs());

    let sum: i64 = diffs.iter().sum();
    return sum;
}

#[test]
fn test_total_distance() {
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];
    assert_eq!(total_distance(left, right), 11);
}
