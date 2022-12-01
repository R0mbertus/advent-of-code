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

    let most_calories: i32 = elves_calories.pop().unwrap();
    let calories_top_3: i32 = most_calories + elves_calories.pop().unwrap() + elves_calories.pop().unwrap();
    println!("The most calories one elve has: {}", most_calories);
    println!("The calories the 3 elves with the most have: {}", calories_top_3)
}
