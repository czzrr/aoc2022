fn priority(item: u8) -> u64 {
    if item >= 97 {
        (item - 96) as u64
    } else {
        (item - 38) as u64
    }
}

fn main() {
    let lines = aoc_io::lines("input.txt").unwrap();

    let s: u64 = lines
        .iter()
        .map(|line| {
            let line: Vec<_> = line.bytes().collect();
            let c1 = &line[0..line.len() / 2];
            let c2 = &line[line.len() / 2..];
            let mut dups = Vec::new();
            for item in c1 {
                if !dups.contains(item) && c2.contains(item) {
                    dups.push(*item);
                }
            }
            let priority_sum: u64 = dups.iter().map(|item| priority(*item)).sum();
            priority_sum
        })
        .sum();

    println!("{}", s);

    let s: u64 = lines
        .chunks(3)
        .map(|group| {
            let e1: Vec<_> = group[0].bytes().collect();
            let e2: Vec<_> = group[1].bytes().collect();
            let e3: Vec<_> = group[2].bytes().collect();
            for item in e1 {
                if e2.contains(&item) && e3.contains(&item) {
                    return item;
                }
            }
            unreachable!()
        })
        .map(|item| priority(item))
        .sum();

    println!("{}", s);
}
