use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Clone)]
struct Beam {
    map: Vec<Vec<char>>,
    cache: HashMap<((i64, i64), Direction), bool>,
}

impl Beam {
    fn try_move(&mut self, pos: (i64, i64), dir: Direction) -> Option<(i64, i64)> {
        let moved = match dir {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::West => (pos.0, pos.1 - 1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
        };

        if moved.0 < 0
            || moved.0 >= self.map.len() as i64
            || moved.1 < 0
            || moved.1 >= self.map[0].len() as i64
        {
            None
        } else {
            Some(moved)
        }
    }

    fn shoot(&mut self, pos: (i64, i64), dir: Direction) {
        if let Some(new_pos) = self.try_move(pos, dir) {
            if self.cache.insert((new_pos, dir), true).is_some() {
                return;
            }

            match self.map[new_pos.0 as usize][new_pos.1 as usize] {
                '/' => {
                    match dir {
                        Direction::North => self.shoot(new_pos, Direction::East),
                        Direction::East => self.shoot(new_pos, Direction::North),
                        Direction::South => self.shoot(new_pos, Direction::West),
                        Direction::West => self.shoot(new_pos, Direction::South),
                    }
                    return;
                }
                '\\' => {
                    match dir {
                        Direction::North => self.shoot(new_pos, Direction::West),
                        Direction::West => self.shoot(new_pos, Direction::North),
                        Direction::South => self.shoot(new_pos, Direction::East),
                        Direction::East => self.shoot(new_pos, Direction::South),
                    }
                    return;
                }
                '-' => {
                    if dir == Direction::North || dir == Direction::South {
                        self.shoot(new_pos, Direction::West);
                        self.shoot(new_pos, Direction::East);
                        return;
                    }
                }
                '|' => {
                    if dir == Direction::West || dir == Direction::East {
                        self.shoot(new_pos, Direction::North);
                        self.shoot(new_pos, Direction::South);
                        return;
                    }
                }
                _ => {}
            }
            self.shoot(new_pos, dir);
        }
    }

    fn count_energy(&self) -> usize {
        let mut visited = HashSet::new();
        self.cache.iter().for_each(|(&(pos, _), &v)| {
            if v {
                visited.insert(pos);
            }
        });
        visited.len()
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Beam {
    Beam {
        map: input.lines().map(|l| l.chars().collect()).collect(),
        cache: HashMap::new(),
    }
}

#[aoc(day16, part1)]
fn part1(input: &Beam) -> usize {
    let mut clone: Beam = input.clone();
    clone.shoot((0, -1), Direction::East);
    clone.count_energy()
}

#[aoc(day16, part2)]
fn part2(input: &Beam) -> usize {
    let do_shoot = |start, dir| {
        let mut clone: Beam = input.clone();
        clone.shoot(start, dir);
        clone.count_energy()
    };

    let mut max = 0;
    for y in 0..input.map.len() as i64 {
        max = max.max(do_shoot((y, -1), Direction::East));
        max = max.max(do_shoot((y, input.map[0].len() as i64), Direction::West));
    }
    for x in 0..input.map[0].len() as i64 {
        max = max.max(do_shoot((-1, x), Direction::South));
        max = max.max(do_shoot((input.map.len() as i64, x), Direction::North));
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT1)), 51);
    }
}
