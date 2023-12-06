use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    let (durations, distances) = input.split_once("\n").unwrap();
    durations
        .split_whitespace()
        .filter_map(|d| d.parse::<u64>().ok())
        .zip(
            distances
                .split_whitespace()
                .filter_map(|d| d.parse::<u64>().ok()),
        )
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &Vec<(u64, u64)>) -> usize {
    let mut total = 1;
    for (time, distance) in input {
        total *= (1..*time)
            .collect::<Vec<u64>>()
            .into_iter()
            .filter(|t| (*t * (*time - *t)) > *distance)
            .count()
    }
    total
}

#[aoc(day6, part2)]
fn part2(input: &Vec<(u64, u64)>) -> usize {
    let duration = input
        .iter()
        .map(|(t, _)| t.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance = input
        .iter()
        .map(|(_, d)| d.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    part1(&vec![(duration, distance)])
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 71503);
    }
}
