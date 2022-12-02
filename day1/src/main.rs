fn main() {
    let lines = aoc_io::lines("input.txt").unwrap();

    let mut calories = Vec::new();
    _ = lines.iter().fold(0, |n: usize, line| {
        if !line.is_empty() {
            n + line.parse::<usize>().unwrap()
        } else {
            calories.push(n);
            0
        }
    });
    calories.sort_by(|x, y| y.cmp(x));
    let max_calories = calories[0];

    println!("{}", max_calories);

    let sum_max_three_calories: usize = calories.iter().take(3).sum();

    println!("{}", sum_max_three_calories);
}
