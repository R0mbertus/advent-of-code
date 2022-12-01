mod days;

use days::{day1};
use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect format, run as: 'cargo run [day number]'");
    }

    let day: u8 = args[1].parse::<u8>().unwrap();
    let (part1, part2) = get_day_solver(day);

    println!("\n=== Day {:02} ===", day);

    let time: Instant = Instant::now();
    part1();
    let part1_ms: f64 = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("  Part 1 time: {:.4} ms", part1_ms);
    
    part2();
    let part2_ms: f64 = (time.elapsed().as_nanos() as f64 / 1_000_000.0) - part1_ms;
    println!("  Part 2 time: {:.4} ms", part2_ms);

}

fn get_day_solver(day: u8) -> (fn(), fn()) {
    match day {
        1   => (day1::solve1, day1::solve2),
        _   => unimplemented!(),
    }
}