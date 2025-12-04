use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse1(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

const NEIGHBORS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[aoc(day4, part1)]
fn part1(input: &[Vec<char>]) -> i64 {
    input.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().fold(0, |acc_inner, (x, p)| {
            acc_inner
                + if p == &'@' {
                    let neighboring_rolls = NEIGHBORS.iter().fold(0, |rolls, (dy, dx)| {
                        let (ny, nx) = ((y as i64 + dy), (x as i64 + dx));
                        rolls
                            + !(ny < 0
                                || ny >= input.len() as i64
                                || nx < 0
                                || nx >= input.len() as i64
                                || input[ny as usize][nx as usize] == '.')
                                as i64
                    });
                    (neighboring_rolls < 4) as i64
                } else {
                    0
                }
        })
    })
}

#[aoc(day4, part2)]
fn part2(input: &Vec<Vec<char>>) -> usize {
    let mut input_mut = input.clone();
    let mut worklist: VecDeque<(usize, usize)> = (0..input.len())
        .flat_map(|y| (0..input[0].len()).map(move |x| (y, x)))
        .collect();
    while let Some((y, x)) = worklist.pop_back() {
        if input_mut[y][x] != 'x' && input_mut[y][x] == '@' {
            let mut neighbors: VecDeque<(usize, usize)> = NEIGHBORS
                .iter()
                .filter_map(|(dy, dx)| {
                    let (ny, nx) = ((y as i64 + dy), (x as i64 + dx));
                    if ny < 0 || ny >= input.len() as i64 || nx < 0 || nx >= input.len() as i64 {
                        None
                    } else {
                        Some((ny as usize, nx as usize))
                    }
                })
                .collect();

            let neighboring_rolls = neighbors.iter().fold(0, |rolls, &(ny, nx)| {
                rolls + (input_mut[ny][nx] == '@') as i64
            });

            if neighboring_rolls < 4 {
                input_mut[y][x] = 'x';
                worklist.append(&mut neighbors);
            }
        }
    }

    input_mut.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&&p| p == 'x').count()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 43);
    }
}
