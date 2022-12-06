use std::fs::read_to_string;
use itertools::Itertools;

fn sliding_window(v: Vec<u8>, n: usize) -> usize {
    for i in n..v.len() {
        let window = v[i-n..i].to_vec();
        if window.len() == window.into_iter().sorted().dedup().count() {
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

    println!("  Part 1: {}", sliding_window(total_signal, 4));
}

pub fn solve2() {
    let total_signal = read_to_string("src/days/input/6.txt")
        .unwrap()
        .as_bytes()
        .to_vec();

    println!("  Part 2: {}", sliding_window(total_signal, 14));
}
