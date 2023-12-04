use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|l| {
            let (_, cards) = l.split_once(": ").unwrap();
            let (scratch, winning) = cards.split_once(" | ").unwrap();
            (
                scratch
                    .split_whitespace()
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect(),
                winning
                    .split_whitespace()
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Vec<(Vec<u32>, Vec<u32>)>) -> u32 {
    input
        .iter()
        .map(|l| {
            let (scratch, winning): (HashSet<&u32>, HashSet<&u32>) =
                (l.0.iter().collect(), l.1.iter().collect());
            let intersect = scratch.intersection(&winning).count() as u32;
            if intersect > 0 {
                2u32.pow(intersect - 1)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &Vec<(Vec<u32>, Vec<u32>)>) -> u32 {
    let winnings: Vec<u32> = input
        .iter()
        .map(|l| {
            let (scratch, winning): (HashSet<&u32>, HashSet<&u32>) =
                (l.0.iter().collect(), l.1.iter().collect());
            return scratch.intersection(&winning).count() as u32;
        })
        .collect();

    let mut cards = vec![1; winnings.len()];

    for (i, winning) in winnings.clone().into_iter().enumerate() {
        if winning > 0 {
            for j in (i + 1)..(cards.len()).min(i + winning as usize + 1) {
                cards[j] += cards[i];
            }
        }
    }

    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )),
            13
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )),
            30
        );
    }
}
