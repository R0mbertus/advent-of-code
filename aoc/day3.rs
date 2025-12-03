use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day3)]
fn parse1(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u32>]) -> i64 {
    input.iter().fold(0, |acc, bank| {
        acc + bank
            .iter()
            .combinations(2)
            .map(|combi| {
                combi
                    .into_iter()
                    .fold(0, |acc_inner, &digit| acc_inner * 10 + digit as i64)
            })
            .max()
            .unwrap()
    })
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u32>]) -> i64 {
    input.iter().fold(0, |acc, bank| {
        acc + (0..12)
            .rev()
            .fold((0, vec![]), |(i, nums), n| {
                let slice = &bank[i..bank.len() - n];
                let (max_i, &max_v) = slice.iter().enumerate().rev().max_by_key(|a| a.1).unwrap();
                (i + max_i + 1, nums.into_iter().chain(vec![max_v]).collect())
            })
            .1
            .iter()
            .fold(0, |acc_inner, &num| acc_inner * 10 + num as i64)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 3121910778619);
    }
}
