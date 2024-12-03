use std::cmp::Ordering;

use crate::advent_day::AdventDay;

pub struct Day02 {
    reports: Vec<Vec<u64>>,
}

impl Day02 {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
        }
    }
}

impl AdventDay for Day02 {
    fn parse(&mut self, input: &str) {
        for line in input.lines() {
            let values: Vec<u64> = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
            self.reports.push(values);
        }
    }

    fn p1(&self) -> String {
        let safe_count = self
            .reports
            .iter()
            .map(|r| is_safe(r))
            .filter(|x| *x)
            .count();
        return safe_count.to_string();
    }

    fn p2(&self) -> String {
        let safe_count = self
            .reports
            .iter()
            .map(|r| is_safe_with_dampener(r))
            .filter(|x| *x)
            .count();
        return safe_count.to_string();
    }
}

fn is_safe(report: &Vec<u64>) -> bool {
    let is_not_monotonic = match report[0].cmp(&report[1]) {
        Ordering::Less => report.windows(2).any(|pair| pair[0] >= pair[1]),
        Ordering::Equal => true,
        Ordering::Greater => report.windows(2).any(|pair| pair[0] <= pair[1]),
    };
    if is_not_monotonic {
        return false;
    }

    let is_diff_too_large = report
        .windows(2)
        .any(|pair| !(1..=3).contains(&pair[0].abs_diff(pair[1])));

    return !is_diff_too_large;
}

fn is_safe_with_dampener(report: &Vec<u64>) -> bool {
    if !is_safe(report) {
        for idx in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(idx);
            if is_safe(&new_report) {
                return true;
            }
        }
        return false;
    }
    return true;
}

#[test]
fn test_is_safe() {
    assert_eq!(is_safe(&vec![7, 6, 4, 2, 1]), true);
    assert_eq!(is_safe(&vec![9, 7, 6, 2, 1]), false);
    assert_eq!(is_safe(&vec![1, 2, 7, 8, 9]), false);
    assert_eq!(is_safe(&vec![1, 3, 2, 4, 5]), false);
    assert_eq!(is_safe(&vec![8, 6, 4, 4, 1]), false);
    assert_eq!(is_safe(&vec![1, 3, 6, 7, 9]), true);
}

#[test]
fn test_is_report_safe_with_dampener() {
    assert_eq!(is_safe_with_dampener(&vec![7, 6, 4, 2, 1]), true);
    assert_eq!(is_safe_with_dampener(&vec![1, 2, 7, 8, 9]), false);
    assert_eq!(is_safe_with_dampener(&vec![9, 7, 6, 2, 1]), false);
    assert_eq!(is_safe_with_dampener(&vec![1, 3, 2, 4, 5]), true);
    assert_eq!(is_safe_with_dampener(&vec![8, 6, 4, 4, 1]), true);
    assert_eq!(is_safe_with_dampener(&vec![1, 3, 6, 7, 9]), true);

    // https://www.reddit.com/r/adventofcode/comments/1h4shdu/2024_day_2_part2_edge_case_finder/
    assert_eq!(
        is_safe_with_dampener(&vec![48, 46, 47, 49, 51, 54, 56]),
        true
    );
    assert_eq!(is_safe_with_dampener(&vec![1, 1, 2, 3, 4, 5]), true);
    assert_eq!(is_safe_with_dampener(&vec![1, 2, 3, 4, 5, 5]), true);
    assert_eq!(is_safe_with_dampener(&vec![5, 1, 2, 3, 4, 5]), true);
    assert_eq!(is_safe_with_dampener(&vec![1, 4, 3, 2, 1]), true);
    assert_eq!(is_safe_with_dampener(&vec![1, 6, 7, 8, 9]), true);
    assert_eq!(is_safe_with_dampener(&vec![1, 2, 3, 4, 3]), true);
}
