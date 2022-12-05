use std::fs::read_to_string;

fn parse_stacks(s: &str) -> Vec<Vec<u8>> {
    let lines = s.lines().collect::<Vec<_>>();
    let char_width = 4;
    let stacks_amount = lines[0].len() / char_width;
    
    let mut stacks = Vec::with_capacity(stacks_amount);
    for _ in 0..stacks_amount {
        stacks.push(Vec::new());
    }

    for i in (0..(lines.len() - 1)).rev() {
        for j in 0..stacks_amount {
            let c = lines[i].as_bytes()[1 + j * char_width];
            if c.is_ascii_alphabetic() {
                stacks[j].push(c);
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
    let done_stacks = instructions.iter()
        .for_each(|v| {
            let (amount, from, to) = (v[0], v[1] - 1, v[2] - 1);
            let from_vec = stacks[from].clone();
            stacks[to].append(&mut from_vec[from_vec.len() - amount..].to_vec());
            stacks[from].truncate(from_vec.len() - amount);
        });
    let tops
    println!("  Part 1: {}", )
}

pub fn solve2() {
    
}
