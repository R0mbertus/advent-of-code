use std::fs::read_to_string;

pub fn solve1() {
    let signals = read_to_string("src/days/input/10.txt")
        .unwrap()
        .replace("addx ", "noop\n");

    let mut x_total: i32 = 1;
    let mut interesting_total = 0;
    let interesting_signals: Vec<usize> = vec![19, 59, 99, 139, 179, 219];

    for (clock, operation) in signals.lines().enumerate() {
        if interesting_signals.contains(&clock) {
            interesting_total += x_total * (clock as i32 + 1);
        }
        if operation != "noop" {
            x_total += operation.parse::<i32>().unwrap();
        }
    }

    println!("  Part 1: {}", interesting_total);
}

pub fn solve2() {
    let signals = read_to_string("src/days/input/10.txt")
        .unwrap()
        .replace("addx ", "noop\n");

    let mut x_total: i32 = 1;
    let mut crt: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];

    for (clock, operation) in signals.lines().enumerate() {
        if operation != "noop" {
            x_total += operation.parse::<i32>().unwrap();
        }
    }

    println!("  Part 2: {}", interesting_total);
}
