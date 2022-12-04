use std::ops::RangeInclusive;

fn parse_ranges(s: &str) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    let ranges: Vec<_> = s
        .split(',')
        .map(|range| {
            let range: Vec<u64> = range.split("-").map(|s| s.parse().unwrap()).collect();
            range[0]..=range[1]
        })
        .collect();
    (ranges[0].clone(), ranges[1].clone())
}

fn main() {
    let lines = aoc_io::lines("input.txt").unwrap();

    let c: usize = lines
        .iter()
        .map(|line| {
            let (r1, r2) = parse_ranges(&line);
            (r1.contains(r2.start()) && r1.contains(r2.end()))
                || (r2.contains(r1.start()) && r2.contains(r1.end()))
        })
        .filter(|b| *b)
        .count();

    println!("{}", c);

    let c: usize = lines
        .iter()
        .map(|line| {
            let (r1, r2) = parse_ranges(&line);
            (r1.contains(r2.start()) || r1.contains(r2.end()))
                || (r2.contains(r1.start()) || r2.contains(r1.end()))
        })
        .filter(|b| *b)
        .count();

    println!("{}", c);
}
