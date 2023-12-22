use aoc_runner_derive::{aoc, aoc_generator};
use array2d::Array2D;

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    let to_usize = |c: Vec<char>| {
        let mut buf = 0;
        for elem in c {
            buf <<= 1;
            buf += (elem == '#') as usize;
        }
        buf
    };

    input
        .split("\n\n")
        .map(|b| {
            let array = Array2D::from_rows(
                &b.lines()
                    .map(|l| l.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>(),
            )
            .unwrap();
            (
                array
                    .as_rows()
                    .into_iter()
                    .map(to_usize)
                    .collect::<Vec<usize>>(),
                array
                    .as_columns()
                    .into_iter()
                    .map(to_usize)
                    .collect::<Vec<usize>>(),
            )
        })
        .collect()
}

fn reflect(elements: &[usize], part2: bool) -> Option<usize> {
    let one_smudge = |l: usize, r: usize| (l ^ r) & ((l ^ r) - 1) == 0;

    'outer: for i in 1..elements.len() {
        let (mut lo, mut hi) = (i, i - 1);
        let mut smudged = false;
        while lo > 0 && hi < (elements.len() - 1) {
            lo -= 1;
            hi += 1;
            if elements[lo] != elements[hi] {
                if part2 && !smudged && one_smudge(elements[lo], elements[hi]) {
                    smudged = true;
                    continue;
                }
                continue 'outer;
            }
        }
        if !part2 || smudged {
            return Some(i);
        }
    }
    None
}

#[aoc(day13, part1)]
fn part1(input: &[(Vec<usize>, Vec<usize>)]) -> usize {
    input
        .iter()
        .map(|(r, c)| {
            if let Some(value) = reflect(c, false) {
                value
            } else {
                reflect(r, false).unwrap_or(0) * 100
            }
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[(Vec<usize>, Vec<usize>)]) -> usize {
    input
        .iter()
        .map(|(r, c)| {
            if let Some(value) = reflect(c, true) {
                value
            } else {
                reflect(r, true).unwrap_or(0) * 100
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT1)), 400);
    }
}
