use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BinaryHeap;

fn setup() -> BinaryHeap<i32> {
    let file = File::open("src/days/input/input1.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .flatten()
        .collect();
    
    let mut elves_calories: BinaryHeap<i32> = BinaryHeap::new();
    let mut sum: i32 = 0;
    for line in lines.iter() {
        match line.parse::<i32>() {
            Ok(num) => sum += num,
            Err(_e) => {
                elves_calories.push(sum);
                sum = 0;
            },
        }
    }

    return elves_calories;
}

pub fn solve1() {
    println!("  Part 1: {}", setup().iter().take(1).sum::<i32>());
}

pub fn solve2() {
    println!("  Part 2: {}", setup().iter().take(3).sum::<i32>());
}
