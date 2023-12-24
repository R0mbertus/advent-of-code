use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

struct Pos(i64, i64, i64);

impl Pos {
    fn from(v: Vec<i64>) -> Self {
        Self(v[0], v[1], v[2])
    }
}

struct Hailstone {
    pos: Pos,
    vel: Pos,
}

impl Hailstone {
    fn calculate_t(&self, other: &Self) -> i64 {
        (other.vel.1 * (self.pos.0 - other.pos.0) - other.vel.0 * (self.pos.1 - other.pos.1))
            / (self.vel.1 * other.vel.0 - self.vel.0 * other.vel.1)
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|l| {
            let (raw_pos, raw_vel) = l.split_once(" @ ").unwrap();
            Hailstone {
                pos: Pos::from(raw_pos.split(", ").map(|s| s.parse().unwrap()).collect()),
                vel: Pos::from(raw_vel.split(", ").map(|s| s.parse().unwrap()).collect()),
            }
        })
        .collect()
}

#[aoc(day24, part1)]
fn part1(input: &[Hailstone]) -> usize {
    let (xy_min, xy_max) = (200000000000000i64, 400000000000000i64);

    // linear algebra >:(
    let mut total = 0;
    for (stone1, stone2) in input.iter().tuple_combinations() {
        if stone1.vel.0 * stone2.vel.1 == stone2.vel.0 * stone1.vel.1 {
            continue;
        }

        let t_stone1 = stone1.calculate_t(stone2);
        let t_stone2 = stone2.calculate_t(stone1);
        let intersect_x = stone1.pos.0 + t_stone1 * stone1.vel.0;
        let intersect_y = stone1.pos.1 + t_stone1 * stone1.vel.1;

        if t_stone1 < 0
            || t_stone2 < 0
            || xy_min > intersect_x
            || xy_max < intersect_x
            || xy_min > intersect_y
            || xy_max < intersect_y
        {
            continue;
        }

        total += 1;
    }
    total
}

#[aoc(day24, part2)]
fn part2(_input: &[Hailstone]) -> usize {
    todo!()
}
