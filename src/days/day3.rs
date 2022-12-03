use std::fs::read_to_string;

pub fn solve1() {
    let priority_total: u32 = read_to_string("src/days/input/input2.txt")
        .unwrap()
        .as_str()
        .split("\n")
        .map(str::as_bytes)
        .map(|s| {
            let (str_l, str_r) = s.split_at(s.len() / 2);
            let dup = str_l.iter().find(|c| str_r.contains(c)).unwrap();
            u32::from((dup - 38) % 58)
        })
        .sum();

        println!("  Part 1: {}", priority_total);
}