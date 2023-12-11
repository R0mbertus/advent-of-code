use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input
        .split('\n')
        .map(|s| {
            let (_, data) = s.split_once(": ").unwrap();
            let colors = data
                .split(|p| p == ';' || p == ',')
                .map(|cube| cube.trim().split_once(' ').unwrap())
                .fold(HashMap::new(), |mut colors, (count, color)| {
                    colors
                        .entry(color)
                        .and_modify(|v: &mut i32| *v = (*v).max(count.parse::<i32>().unwrap()))
                        .or_insert(count.parse::<i32>().unwrap_or_default());
                    colors
                });

            Game {
                red: *colors.get("red").unwrap_or(&0),
                green: *colors.get("green").unwrap_or(&0),
                blue: *colors.get("blue").unwrap_or(&0),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> i32 {
    input
        .iter()
        .enumerate()
        .filter(|(_, game)| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .map(|(number, _)| (number + 1) as i32)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> i32 {
    input
        .iter()
        .map(|game| game.red * game.green * game.blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            8
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            2286
        );
    }
}
