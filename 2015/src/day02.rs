// --- Day 2: I Was Told There Would Be No Math ---

pub type Input = Vec<(usize, usize, usize)>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let dims: Vec<usize> = line
                .split("x")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            (dims[0], dims[1], dims[2])
        })
        .collect()
}

pub fn part1(input: &Input) -> String {
    input
        .iter()
        .map(|(l, w, h)| wrapping_paper_needed(*l, *w, *h))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &Input) -> String {
    input
        .iter()
        .map(|(l, w, h)| ribbon_needed(*l, *w, *h))
        .sum::<usize>()
        .to_string()
}

fn wrapping_paper_needed(l: usize, w: usize, h: usize) -> usize {
    let side1 = l * w;
    let side2 = w * h;
    let side3 = l * h;

    let surface_area = 2 * (side1 + side2 + side3);
    let slack = side1.min(side2).min(side3);

    surface_area + slack
}

fn ribbon_needed(l: usize, w: usize, h: usize) -> usize {
    let smallest_perimeter = [l + w, w + h, l + h].iter().min().unwrap() * 2;
    let volume = l * w * h;

    smallest_perimeter + volume
}

#[test]
fn test_ribbon_needed() {
    assert_eq!(ribbon_needed(2, 3, 4), 34);
    assert_eq!(ribbon_needed(1, 1, 10), 14);
}
