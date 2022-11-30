use std::fs::read_to_string;

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: fn(usize, usize) -> usize,
    operation_val: usize,
    divider: usize,
    true_res: usize,
    false_res: usize,
}

fn function_matcher(op: (&str,&str)) -> fn(usize, usize) -> usize {
    match op {
        ("*", "old") => |old: usize, _num: usize| -> usize {old * old},
        ("*", _) => |old: usize, num: usize| -> usize {old * num},
        ("+", _) => |old: usize, num: usize| -> usize {old + num},
        _ => unreachable!(),
    }
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
        divider: split[3].replace("  Test: divisible by ", "").parse::<usize>().unwrap(),
        true_res: split[4].replace("    If true: throw to monkey ", "").parse::<usize>().unwrap(),
        false_res: split[5].replace("    If false: throw to monkey ", "").parse::<usize>().unwrap(),
    }
}

fn run(monkey_business: &mut Vec<Monkey>, rounds: usize, func: impl Fn(usize) -> usize) -> usize {
    let mut results = vec![0; monkey_business.len()];

    for _ in 0..rounds {
        for i in 0..monkey_business.len() {
            for item in monkey_business[i].items.clone() {
                let op = monkey_business[i].operation;
                let worry = func(op(item, monkey_business[i].operation_val));

                let to_monkey = if worry % monkey_business[i].divider == 0 { 
                    monkey_business[i].true_res 
                } else { 
                    monkey_business[i].false_res 
                };

                monkey_business[to_monkey].items.push(worry);
            }

            results[i] += monkey_business[i].items.len();
            monkey_business[i].items.clear();
        }
    }

    results.sort_by_key(|&x| -(x as isize));

    results[0] * results[1]
}

pub fn solve1() {
    let mut monkey_business: Vec<Monkey> = read_to_string("src/days/input/11.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|m| monkey_parser(m))
        .collect();

    println!("  Part 1: {}", run(&mut monkey_business, 20, |x| x / 3));
}

pub fn solve2() {
    let mut monkey_business: Vec<Monkey> = read_to_string("src/days/input/11.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|m| monkey_parser(m))
        .collect();

    let modulus = monkey_business.iter().map(|m| m.divider).product::<usize>();

    println!("  Part 2: {}", run(&mut monkey_business, 10000, |x| x % modulus));
}
