use aoc_runner_derive::{aoc, aoc_generator};
use array2d::Array2D;

#[aoc_generator(day11, part1)]
fn parse1(input: &str) -> Vec<(i64, i64)> {
    let mut spaced = input
        .lines()
        .flat_map(|l| {
            if !l.contains('#') {
                vec![
                    l.chars().collect::<Vec<char>>(),
                    l.chars().collect::<Vec<char>>(),
                ]
            } else {
                vec![l.chars().collect::<Vec<char>>()]
            }
        })
        .collect::<Vec<Vec<char>>>();

    spaced = Array2D::from_columns(
        &Array2D::from_rows(&spaced)
            .unwrap()
            .as_columns()
            .iter()
            .flat_map(|l| {
                if !l.contains(&'#') {
                    vec![l.clone(), l.clone()]
                } else {
                    vec![l.clone()]
                }
            })
            .collect::<Vec<Vec<char>>>(),
    )
    .unwrap()
    .as_rows();

    let mut galaxies = vec![];
    for (y, row) in spaced.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if v == &'#' {
                galaxies.push((y as i64, x as i64));
            }
        }
    }

    galaxies
}

#[aoc(day11, part1)]
fn part1(input: &Vec<(i64, i64)>) -> i64 {
    let mut total = 0;
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            total += i64::abs(input[i].0 - input[j].0) + i64::abs(input[i].1 - input[j].1)
        }
    }
    total
}

#[aoc_generator(day11, part2)]
fn parse2(input: &str) -> Vec<(i64, i64)> {
    let mut empty_cols = vec![];
    let mut empty_rows = vec![];

    let spaced = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            if !l.contains('#') {
                empty_rows.push(y);
            }
            l.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    Array2D::from_rows(&spaced)
        .unwrap()
        .as_columns()
        .iter()
        .enumerate()
        .for_each(|(x, l)| {
            if !l.contains(&'#') {
                empty_cols.push(x);
            }
        });

    let mut galaxies = vec![];
    let mut added_y = 0;
    for (y, row) in spaced.iter().enumerate() {
        if empty_rows.contains(&y) {
            added_y += 999999;
        }
        let mut added_x = 0;
        for (x, v) in row.iter().enumerate() {
            if empty_cols.contains(&x) {
                added_x += 999999;
            }
            if v == &'#' {
                galaxies.push((y as i64 + added_y, x as i64 + added_x));
            }
        }
    }

    galaxies
}

#[aoc(day11, part2)]
fn part2(input: &Vec<(i64, i64)>) -> i64 {
    let mut total = 0;
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            total += i64::abs(input[i].0 - input[j].0) + i64::abs(input[i].1 - input[j].1)
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(INPUT)), 374);
    }
}
