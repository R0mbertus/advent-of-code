use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse1(input: &str) -> Vec<(String, i64)> {
    input
        .lines()
        .map(|l| {
            let (dir, count) = l.split_at(1);
            (dir.to_string(), count.parse().unwrap())
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[(String, i64)]) -> i64 {
    let mut count_zero = 0;
    input.iter().fold(50, |v, (dir, count)| {
        let value = (v + (count * if dir == "R" { 1 } else { -1 })).rem_euclid(100);
        count_zero += (value == 0) as i64;
        value
    });
    count_zero
}

#[aoc(day1, part2)]
fn part2(input: &[(String, i64)]) -> i64 {
    let mut count_zero = 0;
    input.iter().fold(50, |v, (dir, count)| {
        count_zero += *count / 100;
        let count_rem = count % 100;
        let dir_num = if dir == "R" { 1 } else { -1 };
        let value = v + (count_rem * dir_num);
        if dir_num == 1 {
            count_zero += (value >= 100) as i64;
        } else {
            count_zero += ((value <= 0) && (v != 0)) as i64;
        }
        value.rem_euclid(100)
    });
    count_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&&parse1(INPUT)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&&parse1(INPUT)), 6);
    }
}
