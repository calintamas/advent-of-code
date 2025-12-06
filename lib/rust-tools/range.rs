pub type Range = (usize, usize);

pub fn contains(id: usize, range: Range) -> bool {
    id >= range.0 && id <= range.1
}

pub fn overlaps(range_a: Range, range_b: Range) -> bool {
    range_a.0 <= range_b.1 && range_b.0 <= range_a.1
}

pub fn merge(range_a: Range, range_b: Range) -> Range {
    (range_a.0.min(range_b.0), range_a.1.max(range_b.1))
}

pub fn merge_all(all: Vec<Range>) -> Vec<Range> {
    let mut ranges = all.clone();

    // Sorting by start position we know that if the current range doesn't overlap with the last merged range,
    // it won't overlap with ANY earlier ranges either.
    ranges.sort_unstable_by_key(|r| r.0);

    let mut merged = vec![ranges[0]];

    for current in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if overlaps(*last, current) {
            *last = merge(*last, current);
        } else {
            merged.push(current);
        }
    }

    merged
}
