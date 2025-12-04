// --- Day 2: Gift Shop ---

pub type Input = Vec<(usize, usize)>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .filter_map(|pair| {
            let (start, end) = pair.split_once('-')?;
            Some((start.parse().ok()?, end.parse().ok()?))
        })
        .collect()
}

pub fn part1(input: &Input) -> String {
    let mut sum = 0;
    for &(start, end) in input {
        for value in start..=end {
            if digits_repeat_twice(value) {
                sum += value;
            }
        }
    }
    sum.to_string()
}

pub fn part2(input: &Input) -> String {
    let mut sum = 0;
    for &(start, end) in input {
        for value in start..=end {
            if digits_repeat_n(value) {
                sum += value;
            }
        }
    }
    sum.to_string()
}

fn digits_repeat_twice(value: usize) -> bool {
    let value_str = value.to_string();
    let str_len = value_str.len();

    let mid = str_len / 2;
    let pattern = &value_str[..mid];

    value_str == pattern.repeat(2)
}

fn digits_repeat_n(value: usize) -> bool {
    let value_str = value.to_string();
    let str_len = value_str.len();

    for pattern_len in 1..=str_len / 2 {
        if str_len % pattern_len == 0 {
            let pattern = &value_str[..pattern_len];
            if pattern.repeat(str_len / pattern_len) == value_str {
                return true;
            }
        }
    }

    false
}

#[test]
fn test_is_invalid() {
    assert_eq!(digits_repeat_twice(99), true);
    assert_eq!(digits_repeat_twice(1010), true);
    assert_eq!(digits_repeat_twice(1188511885), true);
    assert_eq!(digits_repeat_twice(38593859), true);
    assert_eq!(digits_repeat_twice(123), false);
    assert_eq!(digits_repeat_twice(9), false);
}

#[test]
fn test_is_invalid_p2() {
    // 2 times
    assert_eq!(digits_repeat_n(12341234), true);
    assert_eq!(digits_repeat_n(99), true);

    // 3 times
    assert_eq!(digits_repeat_n(123123123), true);
    assert_eq!(digits_repeat_n(252525), true);

    // 7 times
    assert_eq!(digits_repeat_n(1111111), true);

    // none
    assert_eq!(digits_repeat_n(9), false);
    assert_eq!(digits_repeat_n(123), false);
}
