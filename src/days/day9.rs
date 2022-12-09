use std::fs::read_to_string;
use itertools::Itertools;

fn get_h_pos(s: &str, prev_pos: (i32, i32)) -> (i32, i32) {
    match s {
        "U" => (prev_pos.0 + 1, prev_pos.1),
        "D" => (prev_pos.0 - 1, prev_pos.1),
        "L" => (prev_pos.0, prev_pos.1 - 1),
        "R" => (prev_pos.0, prev_pos.1 + 1),
        _ => unreachable!(),
    }
}

fn get_t_pos(t_pos: (i32, i32), h_pos: (i32, i32)) -> (i32, i32) {
    let pos_diff = (h_pos.0 - t_pos.0, h_pos.1 - t_pos.1);
    match pos_diff {
        (-2, -1) | (-2, -2) | (-1, -2)  => (t_pos.0 - 1, t_pos.1 - 1),
        (2, -1) | (2, -2) | (1, -2)     => (t_pos.0 + 1, t_pos.1 - 1),
        (2, 1) | (2, 2) | (1, 2)        => (t_pos.0 + 1, t_pos.1 + 1),
        (-2, 1) | (-2, 2) | (-1, 2)     => (t_pos.0 - 1, t_pos.1 + 1),
        (-2, 0) => (t_pos.0 - 1, t_pos.1),
        (2, 0)  => (t_pos.0 + 1, t_pos.1),
        (0, 2)  => (t_pos.0, t_pos.1 + 1),
        (0, -2) => (t_pos.0, t_pos.1 - 1),
        _ => t_pos,
    }
}

pub fn solve1() {
    let path = read_to_string("src/days/input/9.txt")
        .unwrap();

    let mut pos: Vec<(i32, i32)> = vec![(0,0)];
    let mut h_pos = (0,0);
    let mut t_pos = (0,0);

    path
        .lines()
        .for_each(|d| {
            let (dir, times) = d.split_once(" ").unwrap();
            for _ in 0..times.parse::<u8>().unwrap() {
                h_pos = get_h_pos(dir, h_pos);
                t_pos = get_t_pos(t_pos, h_pos);
                pos.push(t_pos);
            }
        });

    println!("  Part 1: {}", pos.into_iter().sorted().dedup().count());
}

pub fn solve2() {
    let path = read_to_string("src/days/input/9.txt")
        .unwrap();

    let mut pos: Vec<(i32, i32)> = vec![(0,0)];
    let mut knots = vec![(0,0); 10];

    path
        .lines()
        .for_each(|d| {
            let (dir, times) = d.split_once(" ").unwrap();
            for _ in 0..times.parse::<u8>().unwrap() {
                knots[0] = get_h_pos(dir, knots[0]);
                for i in 1..10 {
                    knots[i] = get_t_pos(knots[i], knots[i - 1]);
                }
                pos.push(knots[9]);
            }
        });

    println!("  Part 2: {}", pos.into_iter().sorted().dedup().count());
}
