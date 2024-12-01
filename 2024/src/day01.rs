use std::collections::HashMap;

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
        self.left.sort(); // sort in place
        self.right.sort();
    }

    fn p1(&self) -> String {
        let sum = total_distance(self.left.clone(), self.right.clone());
        return sum.to_string();
    }

    fn p2(&self) -> String {
        let sum = similarity_score(self.left.clone(), self.right.clone());
        sum.to_string()
    }
}

fn total_distance(left: Vec<u64>, right: Vec<u64>) -> i64 {
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

fn similarity_score(left: Vec<u64>, right: Vec<u64>) -> u64 {
    let mut scores: Vec<u64> = Vec::new();

    let mut map: HashMap<u64, u64> = HashMap::new();
    for &right_item in right.iter() {
        *map.entry(right_item).or_insert(0) += 1
    }

    for left_item in left.iter() {
        if let Some(count) = map.get(left_item) {
            scores.push(left_item * count);
        }
    }

    return scores.iter().sum();
}

#[test]
fn test_total_distance() {
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];
    assert_eq!(total_distance(left, right), 11);
}

#[test]
fn test_similarity_score() {
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];
    assert_eq!(similarity_score(left, right), 31);
}
