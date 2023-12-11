use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<Vec<i64>>> {
    input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|l| {
            let mut history = vec![(l.clone())];
            while history.last().unwrap().iter().any(|n| *n != 0) {
                history.push(
                    history
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|v| v[1] - v[0])
                        .collect_vec(),
                );
            }
            history
        })
        .collect_vec()
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<Vec<i64>>]) -> i64 {
    input
        .iter()
        .map(|h| h.iter().fold(0, |acc, h| h.last().unwrap() + acc))
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<Vec<i64>>]) -> i64 {
    input
        .iter()
        .map(|h| h.iter().rev().fold(0, |acc, h| h.first().unwrap() - acc))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 2);
    }
}
