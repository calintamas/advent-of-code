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
            .map(|report| match is_report_safe(&report) {
                Ok(()) => true,
                Err(_) => false,
            })
            .filter(|x| *x)
            .count();
        return safe_count.to_string();
    }

    fn p2(&self) -> String {
        "".to_string()
    }
}

fn is_report_safe(report: &Vec<u64>) -> Result<(), &str> {
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

    if has_sign_change(&diffs) {
        return Err("sign change");
    }

    if has_value_outside_range(diffs) {
        return Err("val outside range");
    }

    return Ok(());
}

fn has_sign_change(v: &Vec<i64>) -> bool {
    v.windows(2)
        .map(|w| w[0] * w[1])
        .filter(|&item| item < 0)
        .count()
        > 0
}

fn has_value_outside_range(v: Vec<i64>) -> bool {
    let min_diff = 1;
    let max_diff = 3;
    v.iter()
        .filter(|&item| item.abs() < min_diff || item.abs() > max_diff)
        .count()
        > 0
}

#[test]
fn test_has_sign_change() {
    assert_eq!(has_sign_change(&vec![1, 2, 3, 4, 5]), false);
    assert_eq!(has_sign_change(&vec![1, -2, 3, 0, 5]), true);
}

#[test]
fn test_is_report_safe() {
    assert_eq!(is_report_safe(&vec![7, 6, 4, 2, 1]), Ok(()));
    assert_eq!(
        is_report_safe(&vec![1, 2, 7, 8, 9]),
        Err("val outside range")
    );
    assert_eq!(
        is_report_safe(&vec![9, 7, 6, 2, 1]),
        Err("val outside range")
    );
    assert_eq!(is_report_safe(&vec![1, 3, 2, 4, 5]), Err("sign change"));
    assert_eq!(
        is_report_safe(&vec![8, 6, 4, 4, 1]),
        Err("val outside range")
    );
    assert_eq!(is_report_safe(&vec![1, 3, 6, 7, 9]), Ok(()));
}
