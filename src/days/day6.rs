use std::fs::read_to_string;
use itertools::Itertools;

fn sliding_window(v: Vec<u8>) -> usize {
    for i in 4..v.len() {
        let window = v[i-4..i].to_vec();
        if window.len() == window.into_iter().dedup().count() {
            return i
        }
    }
    return 0;
}

pub fn solve1() {
    let total_signal = read_to_string("src/days/input/6.txt")
        .unwrap()
        .as_bytes()
        .to_vec();

    println!("  Part 1: {}", sliding_window(total_signal));
}

pub fn solve2() {
    let all_string = read_to_string("src/days/input/6.txt")
        .unwrap();
}
