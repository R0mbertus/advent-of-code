use std::fs::read_to_string;
use std::collections::HashMap;

fn parse_dir_sizes(lines: Vec<&str>) -> HashMap<String, usize> {
    let mut dirs: Vec<usize>; = Vec::new();
    let mut current: Vec<usize> = Vec::new();

    for line in lines {
        match line.split_once(" ").unwrap() {
            ("$", "ls") => continue,
            ("$", "cd ..") => ,
            ("$", cd_dir) => ,
            ("dir", _) => continue,
            (size, name) => 
        }
    }
    dirs
}

pub fn solve1() {
    let all_system = read_to_string("src/days/input/7.txt")
        .unwrap()
        .lines()
        .collect();

    let sum_size_100000: usize = parse_dir_sizes(all_system)
        .into_values()
        .collect::<Vec<usize>>()
        .into_iter()
        .filter(|e| *e > 100000)
        .sum();
    
    println!("  Part 1: {}", sum_size_100000);
}

pub fn solve2() {
    let all_system = read_to_string("src/days/input/7.txt")
        .unwrap();

    println!("  Part 2: {}", total_signal);
}