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
            .map(|r| is_report_safe(r, 1))
            .filter(|x| *x)
            .count();
        return safe_count.to_string();
    }

    fn p2(&self) -> String {
        let safe_count = self
            .reports
            .iter()
            .map(|r| is_report_safe(r, 0))
            .filter(|x| *x)
            .count();
        return safe_count.to_string();
    }
}

fn is_report_safe(report: &Vec<u64>, iter_count: usize) -> bool {
    let max_iter_count = 1; // The Problem Dampener allows 1 replacement

    let min_diff = 1;
    let max_diff = 3;

    let diffs = report
        .iter()
        .enumerate()
        .fold(Vec::<i64>::new(), |mut acc, (idx, &item)| {
            if let Some(next_item) = report.get(idx + 1) {
                let diff = item as i64 - *next_item as i64;
                acc.push(diff);
            }
            return acc;
        });

    let mut result = true;
    let mut idx_to_check = Vec::<usize>::new();

    for (idx, &item) in diffs.iter().enumerate() {
        if let Some(next_item) = diffs.get(idx + 1) {
            // sign change
            if item * next_item < 0 {
                if iter_count >= max_iter_count {
                    result = false;
                    break;
                }
                // idx is 2-behind original report vector
                // due to cond. above
                idx_to_check.append(&mut vec![idx, idx + 1, idx + 2]);
                result = false;
            }
        }

        // diff too large
        if item.abs() < min_diff || item.abs() > max_diff {
            if iter_count >= max_iter_count {
                result = false;
                break;
            }
            // idx is 1-behind original report vector
            idx_to_check.append(&mut vec![idx, idx + 1]);
            result = false;
        }
    }

    for &idx in idx_to_check.iter() {
        let mut new_report = report.clone();
        new_report.remove(idx);
        if is_report_safe(&new_report, iter_count + 1) {
            result = true;
            break;
        }
    }

    return result;
}

#[test]
fn test_is_report_safe() {
    assert_eq!(is_report_safe(&vec![7, 6, 4, 2, 1], 1), true);
    assert_eq!(is_report_safe(&vec![1, 2, 7, 8, 9], 1), false);
    assert_eq!(is_report_safe(&vec![9, 7, 6, 2, 1], 1), false);
    assert_eq!(is_report_safe(&vec![1, 3, 2, 4, 5], 1), false);
    assert_eq!(is_report_safe(&vec![8, 6, 4, 4, 1], 1), false);
    assert_eq!(is_report_safe(&vec![1, 3, 6, 7, 9], 1), true);
}

#[test]
fn test_is_report_safe_with_dampener() {
    assert_eq!(is_report_safe(&vec![7, 6, 4, 2, 1], 0), true);
    assert_eq!(is_report_safe(&vec![1, 2, 7, 8, 9], 0), false);
    assert_eq!(is_report_safe(&vec![9, 7, 6, 2, 1], 0), false);
    assert_eq!(is_report_safe(&vec![1, 3, 2, 4, 5], 0), true);
    assert_eq!(is_report_safe(&vec![8, 6, 4, 4, 1], 0), true);
    assert_eq!(is_report_safe(&vec![1, 3, 6, 7, 9], 0), true);

    // https://www.reddit.com/r/adventofcode/comments/1h4shdu/2024_day_2_part2_edge_case_finder/
    assert_eq!(is_report_safe(&vec![48, 46, 47, 49, 51, 54, 56], 0), true);
    assert_eq!(is_report_safe(&vec![1, 1, 2, 3, 4, 5], 0), true);
    assert_eq!(is_report_safe(&vec![1, 2, 3, 4, 5, 5], 0), true);
    assert_eq!(is_report_safe(&vec![5, 1, 2, 3, 4, 5], 0), true);
    assert_eq!(is_report_safe(&vec![1, 4, 3, 2, 1], 0), true);
    assert_eq!(is_report_safe(&vec![1, 6, 7, 8, 9], 0), true);
    assert_eq!(is_report_safe(&vec![1, 2, 3, 4, 3], 0), true);
}
