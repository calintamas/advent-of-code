use crate::advent_day::AdventDay;

pub struct Day07 {
    data: Vec<(isize, Vec<isize>)>,
}

impl Day07 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl AdventDay for Day07 {
    fn parse(&mut self, input: &str) {
        for line in input.lines() {
            let mut values = line.split(": ").into_iter();
            let result: isize = values.next().unwrap().parse::<isize>().unwrap();
            let numbers: Vec<isize> = values
                .next()
                .unwrap()
                .split(" ")
                .filter_map(|x| x.parse::<isize>().ok())
                .collect();
            self.data.push((result, numbers));
        }
    }

    fn p1(&self) -> String {
        self.data
            .iter()
            .fold(0, |mut acc, item| {
                if check_numbers(item.0, item.1.clone(), vec!['+', '*']) {
                    acc += item.0;
                }
                acc
            })
            .to_string()
    }

    fn p2(&self) -> String {
        "".to_string()
    }
}

fn get_combinations(operators: Vec<char>, n: usize) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let mut combinations = Vec::new();
    let smaller_combinations = get_combinations(operators.clone(), n - 1);

    for op in operators {
        for comb in &smaller_combinations {
            let mut new_comb = comb.clone();
            new_comb.push(op);
            combinations.push(new_comb);
        }
    }

    combinations
}

fn check_numbers(result: isize, numbers: Vec<isize>, operators: Vec<char>) -> bool {
    let combinations = get_combinations(operators, numbers.len());

    for comb in combinations.iter() {
        let mut operators = comb.chars().into_iter();

        let mut res = *numbers.get(0).unwrap();
        for num in numbers.iter().skip(1) {
            let op = operators.next();
            match op {
                Some('*') => res *= num,
                Some('+') => res += num,
                _ => {}
            }
        }

        if res == result {
            return true;
        }
    }

    false
}

#[test]
fn test_combinations() {
    let combinations = get_combinations(vec!['+', '*'], 2);
    assert_eq!(combinations, vec!["++", "*+", "+*", "**"]);
}

#[test]
fn test_check_numbers() {
    assert_eq!(check_numbers(190, vec![10, 19], vec!['+', '*']), true);
    assert_eq!(check_numbers(3267, vec![81, 40, 27], vec!['+', '*']), true);
    assert_eq!(
        check_numbers(21037, vec![9, 7, 18, 13], vec!['+', '*']),
        false
    );
}

#[test]
fn test_p1() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let mut day = Day07::new();
    day.parse(input);
    assert_eq!(day.p1(), "3749")
}
