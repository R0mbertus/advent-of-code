use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(Vec<char>, u64)> {
    input
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            (cards.chars().collect(), bid.parse().unwrap())
        })
        .collect()
}

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn compare(a: &(Vec<char>, u64), b: &(Vec<char>, u64), joker: bool) -> std::cmp::Ordering {
    let mut a_freqs = HashMap::new();
    let mut b_freqs = a_freqs.clone();
    for card in CARDS {
        let a_freq = a.0.iter().filter(|c| **c == card).count();
        let b_freq = b.0.iter().filter(|c| **c == card).count();
        if a_freq > 0 {
            a_freqs.insert(card, a_freq);
        }
        if b_freq > 0 {
            b_freqs.insert(card, b_freq);
        }
    }

    if joker && a.0.contains(&'J') {
        let jokers = a.0.iter().filter(|c| **c == 'J').count();
        if jokers != 5 {
            let max_key = a_freqs
                .iter()
                .filter(|(c, _)| **c != 'J')
                .max_by(|c, v| c.1.cmp(v.1))
                .unwrap()
                .0;
            a_freqs.entry(*max_key).and_modify(|c| *c += jokers);
            a_freqs.remove_entry(&'J');
        }
    }
    if joker && b.0.contains(&'J') {
        let jokers = b.0.iter().filter(|c| **c == 'J').count();
        if jokers != 5 {
            let max_key = b_freqs
                .iter()
                .filter(|(c, _)| **c != 'J')
                .max_by(|c, v| c.1.cmp(v.1))
                .unwrap()
                .0;
            b_freqs.entry(*max_key).and_modify(|c| *c += jokers);
            b_freqs.remove_entry(&'J');
        }
    }

    let a_values = a_freqs.values_mut().sorted().rev().collect_vec();
    let b_values = b_freqs.values_mut().sorted().rev().collect_vec();

    for (a_max, b_max) in a_values.into_iter().zip(b_values) {
        #[allow(clippy::all)]
        if a_max > b_max {
            return Ordering::Greater;
        } else if b_max > a_max {
            return Ordering::Less;
        }
    }

    for (a_card, b_card) in a.0.iter().zip(&b.0) {
        let (a_pos, b_pos) = (
            CARDS.iter().position(|c| c == a_card).unwrap(),
            CARDS.iter().position(|c| c == b_card).unwrap(),
        );

        #[allow(clippy::all)]
        if a_pos < b_pos {
            return Ordering::Greater;
        } else if b_pos < a_pos {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

#[aoc(day7, part1)]
fn part1(input: &[(Vec<char>, u64)]) -> u64 {
    let mut values = input.to_owned();
    values.sort_by(|a, b| compare(a, b, false));

    let mut total = 0;
    for (i, value) in values.into_iter().enumerate() {
        total += value.1 * (i as u64 + 1);
    }
    total
}

#[aoc(day7, part2)]
fn part2(input: &[(Vec<char>, u64)]) -> u64 {
    let mut values = input.to_owned();
    values.sort_by(|a, b| compare(a, b, true));

    let mut total = 0;
    for (i, value) in values.into_iter().enumerate() {
        total += value.1 * (i as u64 + 1);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 5905);
    }
}
