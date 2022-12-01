use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .flatten()
        .collect();
    
    let mut elve_max: (i32, i32) = (0, 0);
    let mut sum: i32 = 0;
    let mut elve_num: i32 = 1;
    for line in lines.iter() {
        match line.parse::<i32>() {
            Ok(num) => sum += num,
            Err(_e) => {
                if sum > elve_max.1 {
                    elve_max = (elve_num, sum);
                }
                sum = 0;
                elve_num += 1;
            },
        }
    }

    println!("Elve {} has the most calories: {}", elve_max.0, elve_max.1);
}
