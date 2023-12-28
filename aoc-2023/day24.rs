use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
// use z3::{
//     ast::{Ast, Int},
//     Config, Context, SatResult, Solver,
// };

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

// fn to_z3(values: Vec<i64>, ctx: &Context) -> Vec<Int> {
//     values.iter().map(|el| Int::from_i64(ctx, *el)).collect()
// }

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

// #[aoc(day24, part2)]
// fn part2(input: &[Hailstone]) -> u64 {
//     let ctx = Context::new(&Config::new());
//     let solver = Solver::new(&ctx);
//     let stone_z3 = ["sx", "sy", "sz", "sdx", "sdy", "sdz"]
//         .iter()
//         .map(|el| Int::new_const(&ctx, *el))
//         .collect::<Vec<Int>>();

//     for (i, hailstone) in input.iter().take(3).enumerate() {
//         let hailstone_z3 = to_z3(
//             Vec::from([
//                 hailstone.pos.0,
//                 hailstone.pos.1,
//                 hailstone.pos.2,
//                 hailstone.vel.0,
//                 hailstone.vel.1,
//                 hailstone.vel.2,
//             ]),
//             &ctx,
//         );

//         let time = Int::new_const(&ctx, format!("time{i}"));
//         solver.assert(&time.gt(&Int::from_i64(&ctx, 0)));
//         solver.assert(
//             &(&stone_z3[0] + &stone_z3[3] * &time)
//                 ._eq(&(&hailstone_z3[0] + &hailstone_z3[3] * &time)),
//         );
//         solver.assert(
//             &(&stone_z3[1] + &stone_z3[4] * &time)
//                 ._eq(&(&hailstone_z3[1] + &hailstone_z3[4] * &time)),
//         );
//         solver.assert(
//             &(&stone_z3[2] + &stone_z3[5] * &time)
//                 ._eq(&(&hailstone_z3[2] + &hailstone_z3[5] * &time)),
//         );
//     }

//     assert_eq!(solver.check(), SatResult::Sat);
//     let res = solver
//         .get_model()
//         .unwrap()
//         .eval(&(&stone_z3[0] + &stone_z3[1] + &stone_z3[2]), true)
//         .unwrap()
//         .as_u64()
//         .unwrap();
//     res
// }
