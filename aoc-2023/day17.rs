use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::directed::dijkstra::dijkstra;
use std::ops::Add;

type SuccessorsVec = ((Pos, (i64, i64), usize), u32);

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(i64, i64);

impl Add<(i64, i64)> for Pos {
    type Output = Self;

    fn add(self, other: (i64, i64)) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Pos {
    fn successors(
        &self,
        dir: (i64, i64),
        current_len: usize,
        min_len: usize,
        max_len: usize,
        cities: &Vec<Vec<u32>>,
    ) -> Vec<SuccessorsVec> {
        let mut successors = vec![];
        let mut add_successor = |p: Pos, d: (i64, i64), l: usize| {
            if p.0 >= 0 && p.1 >= 0 && p.0 < cities.len() as i64 && p.1 < cities[0].len() as i64 {
                successors.push(((p, d, l), cities[p.0 as usize][p.1 as usize]));
            }
        };

        if current_len < max_len {
            add_successor(*self + dir, dir, current_len + 1);
        }
        // Technically should have an else for len == 0 (start), but ¯\_(ツ)_/¯
        if current_len >= min_len {
            add_successor(*self + (-dir.1, -dir.0), (-dir.1, -dir.0), 1);
            add_successor(*self + (dir.1, dir.0), (dir.1, dir.0), 1);
        }
        successors
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day17, part1)]
fn part1(input: &Vec<Vec<u32>>) -> u32 {
    dijkstra(
        &(Pos(0, 0), (0, 1), 0),
        |&(pos, dir, current_len)| pos.successors(dir, current_len, 1, 3, input),
        |&(p, _, _)| p == Pos(input.len() as i64 - 1, input[0].len() as i64 - 1),
    )
    .unwrap()
    .1
}

#[aoc(day17, part2)]
fn part2(input: &Vec<Vec<u32>>) -> u32 {
    dijkstra(
        &(Pos(0, 0), (0, 1), 0),
        |&(pos, dir, current_len)| pos.successors(dir, current_len, 4, 10, input),
        |&(p, _, _)| p == Pos(input.len() as i64 - 1, input[0].len() as i64 - 1),
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 102);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT1)), 94);
    }
}
