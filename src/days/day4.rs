use std::fs::read_to_string;

fn find_overlap(s: &str, bool_condition: fn((u32, u32, u32, u32)) -> bool) -> u32 {
    let elf_split: Vec<&str> = s.split(",").collect();
    let elf_l: Vec<u32> = elf_split[0].split("-").map(|s| s.parse::<u32>().unwrap()).collect();
    let elf_r: Vec<u32> = elf_split[1].split("-").map(|s| s.parse::<u32>().unwrap()).collect();

    if bool_condition((elf_l[0], elf_l[1], elf_r[0], elf_r[1])) { 1 } else { 0 }
}

fn complete_overlap(elf_overlap: (u32, u32, u32, u32)) -> bool {
    (elf_overlap.0 <= elf_overlap.2 && elf_overlap.1 >= elf_overlap.3) || 
    (elf_overlap.2 <= elf_overlap.0 && elf_overlap.3 >= elf_overlap.1)
}

fn partial_overlap(elf_overlap: (u32, u32, u32, u32)) -> bool {
    elf_overlap.0 <= elf_overlap.3 && elf_overlap.1 >= elf_overlap.2
}

pub fn solve1() {
    let all_overlap: u32 = read_to_string("src/days/input/4.txt")
        .unwrap()
        .lines()
        .map(|s| find_overlap(s, complete_overlap))
        .sum();

    println!("  Part 1: {}", all_overlap);
}

pub fn solve2() {
    let all_overlap: u32 = read_to_string("src/days/input/4.txt")
        .unwrap()
        .lines()
        .map(|s| find_overlap(s, partial_overlap))
        .sum();

    println!("  Part 2: {}", all_overlap);
}
