// --- Day 3: Perfectly Spherical Houses in a Vacuum ---

use std::collections::HashMap;

use rust_tools::point::Point;

pub type Input = Vec<Point>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => Point::new(0, -1),
            'v' => Point::new(0, 1),
            '>' => Point::new(1, 0),
            '<' => Point::new(-1, 0),
            _ => unreachable!(),
        })
        .collect()
}

pub fn part1(input: &Input) -> String {
    deliver(input, |_| true).to_string()
}

pub fn part2(input: &Input) -> String {
    deliver(input, |idx| idx % 2 == 0).to_string()
}

fn deliver(directions: &Vec<Point>, is_santa: fn(usize) -> bool) -> usize {
    let start = Point { x: 0, y: 0 };

    let mut visited = HashMap::<Point, usize>::new();
    visited.insert(start, 1);

    let mut santa = start;
    let mut robot = start;

    for (idx, &direction) in directions.iter().enumerate() {
        if is_santa(idx) {
            santa += direction;
            *visited.entry(santa).or_insert(0) += 1;
        } else {
            robot += direction;
            *visited.entry(robot).or_insert(0) += 1;
        }
    }

    visited.len()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&parse(&">")), "2");
    assert_eq!(part1(&parse(&"^>v<")), "4");
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse(&"^v")), "3");
    assert_eq!(part2(&parse(&"^>v<")), "3");
}
