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

fn calculate_spots(input: &(Vec<Vec<char>>, (i64, i64)), max_steps: usize) -> usize {
    let mut reachable = HashSet::new();
    let mut seen = HashSet::new();
    let mut dequeue = VecDeque::from([((input.1), 0)]);

    while let Some(((y, x), steps)) = dequeue.pop_front() {
        if steps > max_steps || seen.contains(&(y, x)) {
            continue;
        }

        if steps % 2 == max_steps % 2 {
            reachable.insert((y, x));
        }
        seen.insert((y, x));

        for (new_y, new_x) in [(y + 1, x), (y, x + 1), (y - 1, x), (y, x - 1)] {
            if input.0[new_y as usize % input.0.len()][new_x as usize % input.0[0].len()] != '#' {
                dequeue.push_back(((new_y, new_x), steps + 1));
            }
        }
    }
    reachable.len()
}

fn forward_divided(spots: Vec<usize>, goal_steps: usize, rows: usize) -> usize {
    let y2 = (spots[2] - (spots[1] * 2) + spots[0]) / 2;
    let y1 = spots[1] - spots[0] - y2;
    let y0 = spots[0];

    y0 + y1 * ((goal_steps - rows / 2) / rows) + y2 * ((goal_steps - rows / 2) / rows).pow(2)
}

#[aoc(day21, part1)]
fn part1(input: &(Vec<Vec<char>>, (i64, i64))) -> usize {
    calculate_spots(input, 64)
}

#[aoc(day21, part2)]
fn part2(input: &(Vec<Vec<char>>, (i64, i64))) -> usize {
    let goal_steps = 26501365;
    let rows = input.0.len();
    let edge = rows / 2;

    let spots: Vec<usize> = [0, 1, 2]
        .iter()
        .map(|n| calculate_spots(input, n * rows + edge))
        .collect();

    forward_divided(spots, goal_steps, rows)
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
