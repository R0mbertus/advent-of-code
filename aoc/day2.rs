use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day2)]
fn parse1(input: &str) -> Vec<(i64, i64)> {
    input
        .split(',')
        .map(|l| {
            let (start, end) = l.split('-').collect_tuple().unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(i64, i64)]) -> i64 {
    input.iter().fold(0, |acc, (start, end)| {
        acc + (*start..(*end + 1)).fold(0, |acc_inner, id| {
            let id_string = id.to_string();
            if id_string.len() % 2 == 0
                && id_string[..(id_string.len() / 2)] == id_string[(id_string.len() / 2)..]
            {
                acc_inner + id
            } else {
                acc_inner
            }
        })
    })
}

#[aoc(day2, part2)]
fn part2(input: &[(i64, i64)]) -> i64 {
    input.iter().fold(0, |acc, (start, end)| {
        acc + (*start..(*end + 1)).fold(0, |acc_inner, id| {
            let id_string = id.to_string();
            let pattern =
                (1..id_string.len() / 2 + 1).any(|i| id_string.as_bytes().chunks(i).all_equal());
            if pattern {
                acc_inner + id
            } else {
                acc_inner
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 1227775554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 4174379265);
    }
}
