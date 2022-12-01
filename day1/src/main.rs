use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BinaryHeap;

fn main() {
    let file = File::open("input.txt").unwrap();
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

    println!("Solution 1: {}", elves_calories.iter().take(1).sum::<i32>());
    println!("Solution 2: {}", elves_calories.iter().take(3).sum::<i32>());
}
