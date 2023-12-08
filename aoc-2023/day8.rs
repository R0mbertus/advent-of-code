use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use num_integer::lcm;

#[aoc_generator(day8)]
fn parse(input: &str) -> (Vec<char>, HashMap<String, Vec<String>>) {
    let (moves, nodes) = input.split_once("\n\n").unwrap();

    let mut mappings = HashMap::new();

    nodes.lines().for_each(|l| {
        let (key, values) = l.split_once(" = ").unwrap();
        let left_right = values
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        mappings.insert(key.to_string(), left_right);
    });

    (moves.chars().collect(), mappings)
}

#[aoc(day8, part1)]
fn part1(input: &(Vec<char>, HashMap<String, Vec<String>>)) -> usize {
    let mut current = "AAA";
    let mut steps = 0;
    while current != "ZZZ" {
        let current_dir = input.0[(steps) % input.0.len()];
        current = &input.1.get(current).unwrap()[(current_dir == 'R') as usize];
        steps += 1;
    }
    steps
}

#[aoc(day8, part2)]
fn part2(input: &(Vec<char>, HashMap<String, Vec<String>>)) -> usize {
    input
        .1
        .keys()
        .filter(|s| s.as_bytes()[2] as char == 'A')
        .map(|s| {
            let mut current = s.clone();
            let mut steps = 0;
            while current.as_bytes()[2] as char != 'Z' {
                let current_dir = input.0[(steps) % input.0.len()];
                current = input.1.get(&current).unwrap()[(current_dir == 'R') as usize].clone();
                steps += 1;
            }
            steps
        })
        .fold(1, lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 6);
    }

    const INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT2)), 6);
    }
}
