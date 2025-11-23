use crate::advent_day::AdventDay;

pub struct Day02 {
    boxes: Vec<Vec<usize>>,
}

impl Day02 {
    pub fn new() -> Self {
        Self { boxes: Vec::new() }
    }
}

impl AdventDay for Day02 {
    fn parse(&mut self, input: &str) {
        self.boxes
            .extend(input.lines().filter(|line| !line.is_empty()).map(|line| {
                line.split("x")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect()
            }));
    }

    fn p1(&self) -> String {
        self.boxes
            .iter()
            .fold(0, |acc, item| {
                let l = item[0];
                let w = item[1];
                let h = item[2];
                acc + wrapping_paper_needed(l, w, h)
            })
            .to_string()
    }

    fn p2(&self) -> String {
        self.boxes
            .iter()
            .fold(0, |acc, item| {
                let l = item[0];
                let w = item[1];
                let h = item[2];
                acc + ribbon_needed(l, w, h)
            })
            .to_string()
    }
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
