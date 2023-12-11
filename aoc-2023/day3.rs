use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day3)]
fn parse(input: &str) -> HashMap<(usize, usize), (char, Vec<u32>)> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let reg = Regex::new(r"\d+").unwrap();

    let numbers: Vec<(usize, usize, u32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            reg.find_iter(s)
                .map(|m| (m.start(), y, m.as_str().parse::<u32>().unwrap()))
                .collect::<Vec<(usize, usize, u32)>>()
        })
        .collect();

    let mut symbols_numbers: HashMap<(usize, usize), (char, Vec<u32>)> = HashMap::new();
    numbers.iter().for_each(|n| {
        if let Some(symbol) = symbol_around(&grid, *n) {
            let value = symbols_numbers
                .entry(symbol.0)
                .or_insert_with(|| (symbol.1, vec![]));
            value.1.push(n.2);
        }
    });

    symbols_numbers
}

fn symbol_around(grid: &[Vec<char>], num: (usize, usize, u32)) -> Option<((usize, usize), char)> {
    let mut condition = num.2;
    let mut pos = (num.0, num.1);
    while condition != 0 {
        condition /= 10;
        for neigbor in [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] {
            let (new_x, new_y) = (pos.0 as i32 - neigbor.0, pos.1 as i32 - neigbor.1);
            #[allow(clippy::all)]
            let character = grid
                .get(new_y as usize)
                .and_then(|l| Some(l.get(new_x as usize)))
                .unwrap_or(None);
            if let Some(symbol) = character {
                if *symbol != '.' && !symbol.is_ascii_digit() {
                    return Some(((new_x as usize, new_y as usize), *symbol));
                }
            }
        }
        pos.0 += 1;
    }
    None
}

#[aoc(day3, part1)]
fn part1(input: &HashMap<(usize, usize), (char, Vec<u32>)>) -> u32 {
    input.values().map(|(_, v)| v.iter().sum::<u32>()).sum()
}

#[aoc(day3, part2)]
fn part2(input: &HashMap<(usize, usize), (char, Vec<u32>)>) -> u32 {
    input
        .values()
        .filter(|(s, v)| s == &'*' && v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )),
            4361
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )),
            467835
        );
    }
}
