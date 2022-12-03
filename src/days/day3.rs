use std::fs::read_to_string;

fn get_same_chars(str_l: Vec<u8>, str_r: Vec<u8>) -> Vec<u8> {
    str_l.iter()
        .copied()
        .filter(|c| str_r.contains(c))
        .collect()
}

fn priority(c: u8)-> u32 {
    if c.is_ascii_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
}

pub fn solve1() {
    let priority_total: u32 = read_to_string("src/days/input/input3.txt")
        .unwrap()
        .as_str()
        .lines()
        .map(|s| {
            let (str_l, str_r) = s.split_at(s.len()/2);
            get_same_chars(str_l.as_bytes().to_vec(), str_r.as_bytes().to_vec())
        })
        .map(|v| priority(v[0]))    //only first occurence ig
        .sum();
        
    println!("  Part 1: {}", priority_total);
}

pub fn solve2() {
    let priority_total: u32 = read_to_string("src/days/input/input3.txt")
        .unwrap()
        .as_str()
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|s| {
            get_same_chars(s[0].as_bytes().to_vec(), get_same_chars(s[1].as_bytes().to_vec(), s[2].as_bytes().to_vec()))
        })
        .map(|v| priority(v[0]))    //only first occurence ig x2
        .sum();

    println!("  Part 2: {}", priority_total);
}