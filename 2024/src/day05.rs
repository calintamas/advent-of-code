use std::collections::HashMap;

use crate::advent_day::AdventDay;

pub struct Day05 {
    lt_rules: HashMap<usize, Vec<usize>>,
    gt_rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

impl Day05 {
    pub fn new() -> Self {
        Self {
            lt_rules: HashMap::new(),
            gt_rules: HashMap::new(),
            updates: Vec::new(),
        }
    }
}

enum ParseMode {
    Rule,
    Update,
}

impl AdventDay for Day05 {
    fn parse(&mut self, input: &str) {
        let mut mode = ParseMode::Rule;
        for line in input.lines() {
            if line == "" {
                mode = ParseMode::Update;
                continue;
            }
            match mode {
                ParseMode::Rule => {
                    let values: Vec<usize> = line
                        .split("|")
                        .filter_map(|x| x.parse::<usize>().ok())
                        .collect();
                    let a = values.get(0).unwrap();
                    let b = values.get(1).unwrap();
                    self.lt_rules.entry(*a).or_insert(Vec::new()).push(*b);
                    self.gt_rules.entry(*b).or_insert(Vec::new()).push(*a);
                }
                ParseMode::Update => {
                    let values: Vec<usize> = line
                        .split(",")
                        .filter_map(|x| x.parse::<usize>().ok())
                        .collect();
                    self.updates.push(values);
                }
            }
        }
    }

    fn p1(&self) -> String {
        let mut sum = 0;

        for update in self.updates.iter() {
            let mut valid = true;
            for i in 0..update.len() {
                let page = update.get(i).unwrap();

                let mut other_pages = update.clone();
                other_pages.drain(0..i + 1);

                for p in other_pages.iter() {
                    if let Some(lt_rules) = self.lt_rules.get(page) {
                        if !lt_rules.contains(p) {
                            valid = false;
                            break;
                        }
                    } else if let Some(gt_rules) = self.gt_rules.get(page) {
                        if gt_rules.contains(p) {
                            valid = false;
                            break;
                        }
                    }
                }
            }
            if valid {
                sum += update.get(update.len() / 2).unwrap();
            }
        }

        sum.to_string()
    }

    fn p2(&self) -> String {
        "".to_string()
    }
}

#[test]
fn test_sum_middle() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let mut day = Day05::new();
    day.parse(input);
    let sum = day.p1();
    assert_eq!(sum, "143");
}
