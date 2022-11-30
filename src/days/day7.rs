use std::fs::read_to_string;
use std::collections::HashMap;

fn parse_dir_sizes(lines: Vec<&str>) -> HashMap<String, u32> {
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut current_dir_stack: Vec<String> = Vec::new();

    for line in lines {
        match line.split_once(" ").unwrap() {
            ("$", "ls") => continue,
            ("$", "cd ..") => {current_dir_stack.pop();},
            ("$", "cd /") => {current_dir_stack.push("/".to_string());}
            ("$", cd_dir) => {current_dir_stack.push(format!("{}{}/", current_dir_stack.last().unwrap(), cd_dir.split_once(" ").unwrap().1));},
            ("dir", _) => continue,
            (size, _) => {
                for dir in current_dir_stack.clone().into_iter() {
                    let val = *dirs.entry(dir.clone()).or_insert(0);
                    dirs.insert(dir, val + size.parse::<u32>().ok().unwrap());
                }
            },
        }
    }
    dirs
}

pub fn solve1() {
    let all_system = read_to_string("src/days/input/7.txt")
        .unwrap();

    let sum_size_100000: u32 = parse_dir_sizes(all_system.lines().collect())
        .into_values()
        .collect::<Vec<u32>>()
        .into_iter()
        .filter(|e| *e <= 100000)
        .sum();

    println!("  Part 1: {}", sum_size_100000);
}

pub fn solve2() {
    let all_system = read_to_string("src/days/input/7.txt")
        .unwrap();

    let all_dirs: HashMap<String, u32> = parse_dir_sizes(all_system.lines().collect());

    let minimum_dir: u32 = all_dirs
        .clone()
        .into_values()
        .collect::<Vec<u32>>()
        .into_iter()
        .filter(|e| *e >= (all_dirs["/"] - 40000000))
        .min()
        .unwrap();

    println!("  Part 2: {}", minimum_dir);
}