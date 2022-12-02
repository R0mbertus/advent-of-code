use std::fs::read_to_string;

pub fn solve1() {
    let points_total: i32 = read_to_string("src/days/input/input2.txt")
        .unwrap()
        .as_str()
        .split("\n")
        .map(|s| {
            match s {
                "A X" => 4,
                "A Y" => 8,
                "A Z" => 3,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 7,
                "C Y" => 2,
                "C Z" => 6,
                _     => 0,
            }
        })
        .sum();

    println!("  Part 1: {}", points_total);
}

pub fn solve2() {
    let points_total: i32 = read_to_string("src/days/input/input2.txt")
        .unwrap()
        .as_str()
        .split("\n")
        .map(|s| {
            match s {
                "A X" => 3,
                "A Y" => 4,
                "A Z" => 8,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 2,
                "C Y" => 6,
                "C Z" => 7,
                _     => 0,
            }
        })
        .sum();

    println!("  Part 2: {}", points_total);
}
