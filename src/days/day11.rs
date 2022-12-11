use std::fs::read_to_string;

struct Monkey {
    items: Vec<usize>,
    operation: fn(usize, usize) -> usize,
    operation_val: usize,
    bool_cond: fn(usize, usize, usize, usize) -> usize,
    divider: usize,
    true_res: usize,
    false_res: usize,
}

fn function_matcher(op: (&str,&str)) -> fn(usize, usize) -> usize {
    match op {
        ("*", "old") => |old: usize, _num: usize| -> usize {old * old},
        ("*", _) => |old: usize, num: usize| -> usize {old * num},
        ("+", "old") => |old: usize, _num: usize| -> usize {old + old},
        ("+", _) => |old: usize, num: usize| -> usize {old + num},
        _ => unreachable!(),
    }
}

fn bool_cond_parser(num: usize, divider: usize, true_res: usize, false_res: usize) -> usize  {
    if num % divider == 0 {true_res} else {false_res}
}

fn monkey_parser(m: &str) -> Monkey {
    let split = m.lines().collect::<Vec<&str>>();
    
    let items_parsed = split[1]
        .replace("  Starting items: ", "")
        .split(", ")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let operation_parsed = split[2]
        .replace("  Operation: new = old ", "");
    
    let operation_bind = operation_parsed
        .split_once(" ")
        .unwrap();
    
    Monkey {
        items: items_parsed,
        operation: function_matcher(operation_bind),
        operation_val: operation_bind.1.parse::<usize>().unwrap_or(0),
        bool_cond: bool_cond_parser,
        divider: split[3].replace("  Test: divisible by ", "").parse::<usize>().unwrap(),
        true_res: split[4].replace("    If true: throw to monkey ", "").parse::<usize>().unwrap(),
        false_res: split[5].replace("    If false: throw to monkey ", "").parse::<usize>().unwrap(),
    }
}

pub fn solve1() {
    let monkey_business: Vec<Monkey> = read_to_string("src/days/input/11.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|m| monkey_parser(m))
        .collect();

    let mut current_monkey_num = 0;
    for _ in 0..20 {
        let current_monkey = monkey_business[current_monkey_num];
        for item in current_monkey.items {
            
        }
    }

    //println!("  Part 1: {}", signals);
}

pub fn solve2() {
    // let signals = read_to_string("src/days/input/11.txt")
    //     .unwrap();

    // println!("  Part 2: {}", signals);
}
