use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day21)]
fn parse(input: &str) -> (Vec<Vec<char>>, (i64, i64)) {
    let garden: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = garden
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&elem| elem == 'S')
                .map(|x| (y as i64, x as i64))
        })
        .unwrap();
    (garden, start)
}

#[aoc(day21, part1)]
fn part1(input: &(Vec<Vec<char>>, (i64, i64))) -> usize {
    let mut reachable = HashSet::new();
    let mut seen = HashSet::new();
    let mut dequeue = VecDeque::from([((input.1), 64)]);

    while let Some(((y, x), steps)) = dequeue.pop_front() {
        if steps % 2 == 0 {
            reachable.insert((y, x));
        }
        if steps == 0 {
            continue;
        }

        for (new_y, new_x) in [(y + 1, x), (y, x + 1), (y - 1, x), (y, x - 1)] {
            if new_y >= 0
                && new_y < input.0.len() as i64
                && new_x >= 0
                && new_x < input.0[0].len() as i64
                && input.0[new_y as usize][new_x as usize] != '#'
                && !seen.contains(&(new_y, new_x))
            {
                seen.insert((new_y, new_x));
                dequeue.push_back(((new_y, new_x), steps - 1));
            }
        }
    }
    reachable.len()
}

#[aoc(day21, part2)]
fn part2(_input: &(Vec<Vec<char>>, (i64, i64))) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 42);
    }
}
