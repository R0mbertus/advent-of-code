use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1, part1)]
fn parse1(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|r| r.chars().filter(|c| c.is_digit(10)).collect::<String>())
        .map(|s| {
            let first = s.chars().clone().next().unwrap();
            let last = s.chars().last().unwrap();
            if s.len() < 1 {
                first.to_digit(10).unwrap()
            } else {
                format!("{}{}", first, last).parse().unwrap()
            }
        })
        .collect()
}

#[aoc_generator(day1, part2)]
fn parse2(input: &str) -> Vec<u32> {
    let nums = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "th3ee"),
        ("four", "f4ur"),
        ("five", "fi5e"),
        ("six", "s6x"),
        ("seven", "se7en"),
        ("eight", "ei8ht"),
        ("nine", "ni9e"),
    ];

    let mut parsed = input.to_string();
    for (num, replacement) in nums {
        parsed = parsed.replace(num, replacement);
    }

    parse1(parsed.as_str())
}

#[aoc(day1, part1)]
fn part1(input: &Vec<u32>) -> u32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<u32>) -> u32 {
    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&&parse1(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            )),
            142
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&&parse2(
                "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            )),
            281
        );
    }
}
