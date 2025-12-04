// --- Day 3: Lobby ---

pub type Input = Vec<Vec<usize>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|char| char.to_digit(10).map(|digit| digit as usize))
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> String {
    input
        .iter()
        .map(|power_bank| max_joltage(&power_bank, 2))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &Input) -> String {
    input
        .iter()
        .map(|power_bank| max_joltage(&power_bank, 12))
        .sum::<usize>()
        .to_string()
}

fn max_joltage(values: &[usize], battery_size: usize) -> usize {
    let mut bank = &values[..];
    let mut joltage = 0;

    let n = battery_size - 1;

    for remaining_digits in 0..n {
        // Take all digits except last "n" digits we'll still need to fill the battery up to "battery_size".
        let potential_digits = &bank[..bank.len() - (n - remaining_digits)];
        let (idx, digit) = max_with_index(potential_digits);
        bank = &bank[idx + 1..];
        joltage = joltage * 10 + digit;
    }

    let (_, final_digit) = max_with_index(&bank);
    joltage = joltage * 10 + final_digit;

    joltage
}

fn max_with_index(values: &[usize]) -> (usize, usize) {
    let val = values.iter().max().unwrap();
    let idx = values.iter().position(|item| item == val).unwrap();
    (idx, *val)
}

#[test]
fn test_max_joltage() {
    assert_eq!(max_joltage(&parse("987654321111111")[0], 2), 98);
    assert_eq!(max_joltage(&parse("811111111111119")[0], 2), 89);
    assert_eq!(max_joltage(&parse("234234234234278")[0], 2), 78);
    assert_eq!(max_joltage(&parse("818181911112111")[0], 2), 92);
}

#[test]
fn test_max_joltage_2() {
    assert_eq!(max_joltage(&parse("987654321111111")[0], 12), 987654321111);
    assert_eq!(max_joltage(&parse("811111111111119")[0], 12), 811111111119);
    assert_eq!(max_joltage(&parse("234234234234278")[0], 12), 434234234278);
    assert_eq!(max_joltage(&parse("818181911112111")[0], 12), 888911112111);
}
