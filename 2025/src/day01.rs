// --- Day 1: Secret Entrance ---

pub type Input = Vec<isize>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| {
            let (prefix, num_str) = line.trim().split_at(1);
            let num = num_str.parse::<isize>().ok().unwrap();
            match prefix {
                "L" => Some(-num),
                "R" => Some(num),
                _ => None,
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> String {
    let mut pointer = 50;
    let mut password = 0; // how many times we stopped at 0

    for &value in input {
        pointer = (pointer + value).rem_euclid(100);
        if pointer == 0 {
            password += 1
        }
    }

    password.to_string()
}

pub fn part2(input: &Input) -> String {
    let mut pointer = 50;
    let mut password = 0; // how many times we crossed through or stopped at 0

    for &offset in input {
        let prev_pointer = pointer;

        if offset > 0 {
            // going forward
            password += (pointer + offset) / 100;
        } else {
            // going backward
            let mut offset_cpy = offset.abs();
            if offset_cpy > 100 {
                // for full rotations, we only care about how many times we crossed through 0
                password += offset_cpy / 100;
                // after we counted that, we can normalize the value to [0,99]
                offset_cpy = offset_cpy % 100;
            }
            let new_pointer = pointer - offset_cpy;
            if new_pointer <= 0 && prev_pointer != 0 {
                password += 1
            }
        }
        pointer = (pointer + offset).rem_euclid(100);
    }

    password.to_string()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&parse(
            "L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82"
        )),
        "3"
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(&parse(
            "L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82"
        )),
        "6"
    );
    assert_eq!(part2(&parse("R1000")), "10");
    assert_eq!(
        part2(&parse(
            // test case from https://www.reddit.com/r/adventofcode/comments/1pbha22/comment/nrqey3y
            "R50
            R50
            L50
            L50
            R75
            L50"
        )),
        "4"
    );
}
