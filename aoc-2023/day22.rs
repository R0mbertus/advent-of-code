use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Brick {
    bottom_end: (usize, usize, usize),
    top_end: (usize, usize, usize),
}

impl Brick {
    fn add(&mut self, other: usize) {
        self.bottom_end.2 += other;
        self.top_end.2 += other;
    }

    fn sub(&mut self, other: usize) {
        self.bottom_end.2 -= other;
        self.top_end.2 -= other;
    }

    // fn collision(&self, other: &Self) -> bool {
    //     self.bottom_end >= other.bottom_end && self.bottom_end <= other.top_end
    //         || self.top_end >= other.bottom_end && self.top_end <= other.top_end
    // }
    fn collision(&self, other: &Self) -> bool {
        let point_in_range = |p, range: (usize, usize)| p >= range.0 && p <= range.1;

        (point_in_range(self.bottom_end.2, (other.bottom_end.2, other.top_end.2))
            || point_in_range(self.top_end.2, (other.bottom_end.2, other.top_end.2))
            || point_in_range(other.bottom_end.2, (self.bottom_end.2, self.top_end.2))
            || point_in_range(other.top_end.2, (self.bottom_end.2, self.top_end.2)))
            && (point_in_range(self.bottom_end.0, (other.bottom_end.0, other.top_end.0))
                || point_in_range(self.top_end.0, (other.bottom_end.0, other.top_end.0))
                || point_in_range(other.bottom_end.0, (self.bottom_end.0, self.top_end.0))
                || point_in_range(other.top_end.0, (self.bottom_end.0, self.top_end.0)))
            && (point_in_range(self.bottom_end.1, (other.bottom_end.1, other.top_end.1))
                || point_in_range(self.top_end.1, (other.bottom_end.1, other.top_end.1))
                || point_in_range(other.bottom_end.1, (self.bottom_end.1, self.top_end.1))
                || point_in_range(other.top_end.1, (self.bottom_end.1, self.top_end.1)))
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (bottom, top) = line.split_once('~').unwrap();

            Brick {
                bottom_end: bottom
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
                top_end: top
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            }
        })
        .sorted_by_key(|brick| brick.bottom_end.2)
        .collect()
}

fn drop_bricks(
    bricks: &Vec<Brick>,
    supports: &mut HashMap<Brick, Vec<Brick>>,
    dependencies: &mut HashMap<Brick, Vec<Brick>>,
) -> Vec<Brick> {
    let mut dropped: Vec<Brick> = Vec::new();
    for brick in bricks {
        let mut falling = brick.clone();
        while falling.bottom_end.2 > 1 {
            falling.sub(1);

            let collisions: Vec<Brick> = dropped
                .clone()
                .into_iter()
                .filter(|b| falling.collision(b))
                .collect();
            if !collisions.is_empty() {
                falling.add(1);
                dependencies.insert(falling.clone(), collisions.clone());
                for collision in collisions {
                    supports.entry(collision).or_default().push(falling.clone())
                }
                break;
            }
        }
        dropped.push(falling);
    }
    dropped
}

#[aoc(day22, part1)]
fn part_1(input: &Vec<Brick>) -> usize {
    let mut supports = HashMap::new();
    let mut dependencies = HashMap::new();

    let dropped = drop_bricks(input, &mut supports, &mut dependencies);

    let mut total = 0;
    for brick in dropped {
        if let Some(values) = supports.get(&brick) {
            if values
                .iter()
                .any(|b| dependencies.get(b).unwrap().len() <= 1)
            {
                continue;
            }
        }
        total += 1
    }
    total
}

#[aoc(day22, part2)]
fn part_2(input: &Vec<Brick>) -> usize {
    let mut supports = HashMap::new();
    let mut dependencies = HashMap::new();

    let dropped = drop_bricks(input, &mut supports, &mut dependencies);

    let mut total = 0;
    for brick in dropped {
        if supports.get(&brick).is_none() {
            continue;
        }

        let mut fallen = Vec::new();
        let mut queue = VecDeque::from([brick]);

        while let Some(b) = queue.pop_back() {
            fallen.push(b.clone());

            for sb in supports.get(&b).unwrap_or(&Vec::new()) {
                if dependencies
                    .get(sb)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .all(|db| fallen.iter().contains(db))
                {
                    queue.push_front(sb.clone());
                }
            }
        }

        total += fallen.len() - 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "1,0,1~1,2,1
0,2,3~2,2,3
2,0,5~2,2,5
0,0,4~0,2,4
0,1,6~2,1,6
0,0,2~2,0,2
1,1,8~1,1,9";

        assert_eq!(part_1(&parse(input)), 5);
    }
}

/*
A -> B, C   Brick { bottom_end: (1, 0, 1), top_end: (1, 2, 1) }: [Brick { bottom_end: (0, 0, 2), top_end: (2, 0, 2) }, Brick { bottom_end: (0, 2, 2), top_end: (2, 2, 2) }]}
B -> D, E   Brick { bottom_end: (0, 0, 2), top_end: (2, 0, 2) }: [Brick { bottom_end: (0, 0, 3), top_end: (0, 2, 3) }, Brick { bottom_end: (2, 0, 3), top_end: (2, 2, 3) }],
C -> D, E   Brick { bottom_end: (0, 2, 2), top_end: (2, 2, 2) }: [Brick { bottom_end: (0, 0, 3), top_end: (0, 2, 3) }, Brick { bottom_end: (2, 0, 3), top_end: (2, 2, 3) }],
D -> F      Brick { bottom_end: (0, 0, 3), top_end: (0, 2, 3) }: [Brick { bottom_end: (0, 1, 4), top_end: (2, 1, 4) }],
E -> F      Brick { bottom_end: (2, 0, 3), top_end: (2, 2, 3) }: [Brick { bottom_end: (0, 1, 4), top_end: (2, 1, 4) }],
F -> G      Brick { bottom_end: (0, 1, 4), top_end: (2, 1, 4) }: [Brick { bottom_end: (1, 1, 5), top_end: (1, 1, 6) }],
*/
