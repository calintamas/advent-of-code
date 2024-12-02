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

fn total_distance(mut left: Vec<u64>, mut right: Vec<u64>) -> u64 {
    left.sort(); // sort in place
    right.sort();

    left.iter()
        .enumerate()
        .map(|(idx, &left_item)| left_item.abs_diff(right[idx]))
        .sum()
}

fn similarity_score(left: Vec<u64>, right: Vec<u64>) -> u64 {
    let counts: HashMap<u64, u64> = right
        .iter()
        // accumulate into a HashMap
        .fold(HashMap::new(), |mut acc, &right_item| {
            // .or_insert returns a mutable reference to the value in entry
            // * used to dereference it
            *acc.entry(right_item).or_insert(0) += 1;
            acc
        });

    left.iter()
        .map(|&left_item| left_item * counts.get(&left_item).unwrap_or(&0))
        .sum()
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
