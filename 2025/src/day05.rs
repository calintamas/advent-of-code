// --- Day 5: Cafeteria ---

use rust_tools::range::{contains, merge_all, Range};

pub struct Input {
    pub ranges: Vec<Range>,
    pub ids: Vec<usize>,
}

pub fn parse(input: &str) -> Input {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<usize> = line.split('-').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let ids = parts[1].lines().map(|line| line.parse().unwrap()).collect();

    Input { ranges, ids }
}

pub fn part1(input: &Input) -> String {
    let mut count = 0;
    for &id in &input.ids {
        for &range in &input.ranges {
            if contains(id, range) {
                count += 1;
                break;
            }
        }
    }
    count.to_string()
}

pub fn part2(input: &Input) -> String {
    let ranges = merge_all(input.ranges.clone());

    let mut sum = 0;
    for &range in &ranges {
        sum += (range.1 - range.0) + 1;
    }

    sum.to_string()
}

#[test]
fn test_p1() {
    assert_eq!(
        part1(&parse(
            &"3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        )),
        "3"
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        part2(&parse(
            &"3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        )),
        "14"
    );
}
