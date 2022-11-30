use std::fs::read_to_string;
use std::str;

fn parse_stacks(s: &str) -> Vec<Vec<u8>> {
    let lines = s.lines().rev().map(str::as_bytes);
    let stacks_amount = 9;
    
    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); stacks_amount];

    for line in lines {
        for stack_num in 0..stacks_amount {
            let c = line[1 + stack_num * 4];
            if c.is_ascii_alphabetic() {
                stacks[stack_num].push(c);
            }
        }
    }
    stacks
}

fn parse_instructions(s: &str) -> Vec<Vec<usize>> {
    s.lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

pub fn solve1() {
    let all_string = read_to_string("src/days/input/5.txt")
        .unwrap();
    let (stack_data, instruction_data) = all_string.split_once("\n\n")
        .unwrap();
    let mut stacks = parse_stacks(stack_data);

    let instructions = parse_instructions(instruction_data);
    instructions.iter().for_each(|v| {
        let (amount, from, to) = (v[0], v[1] - 1, v[2] - 1);
        for _ in 0..amount {
            let crate_to_push = stacks[from].pop().unwrap();
            stacks[to].push(crate_to_push);
        }
    });

    let tops = stacks.into_iter().map(|s| s[s.len()-1]).collect::<Vec<u8>>();
    println!("  Part 1: {}", String::from_utf8(tops.clone()).unwrap());
}

pub fn solve2() {
    let all_string = read_to_string("src/days/input/5.txt")
        .unwrap();
    let (stack_data, instruction_data) = all_string.split_once("\n\n")
        .unwrap();
    let mut stacks = parse_stacks(stack_data);

    let instructions = parse_instructions(instruction_data);
    instructions.iter().for_each(|v| {
        let (amount, from, to) = (v[0], v[1] - 1, v[2] - 1);
        let from_stack = stacks[from].clone();
        stacks[to].append(&mut from_stack[from_stack.len() - amount..].to_vec());
        stacks[from].truncate(from_stack.len() - amount);
    });

    let tops = stacks.into_iter().map(|s| s[s.len()-1]).collect::<Vec<u8>>();
    println!("  Part 2: {}", String::from_utf8(tops.clone()).unwrap());
}
