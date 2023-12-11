use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let (seeds_str, mappings_str) = input.split_once("\n\n").unwrap();
    let reg = Regex::new(r"\d+").unwrap();
    let seeds = reg
        .find_iter(seeds_str)
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect();

    let mappings = mappings_str
        .split("\n\n")
        .map(|b| {
            b.split('\n')
                .map(|l| {
                    reg.find_iter(l)
                        .map(|m| m.as_str().parse::<i64>().unwrap())
                        .collect()
                })
                .filter(|v: &Vec<i64>| v.len() == 3)
                .collect()
        })
        .collect();

    (seeds, mappings)
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<i64>, Vec<Vec<Vec<i64>>>)) -> i64 {
    let mut seeds = input.0.clone();

    input.1.clone().iter().for_each(|b| {
        let seeds_curr = seeds.clone();
        b.iter().for_each(|l| {
            let delta = l[1] - l[0];
            for (i, seed) in seeds_curr.iter().enumerate() {
                if *seed >= l[1] && *seed < (l[1] + l[2]) {
                    seeds[i] = *seed - delta;
                }
            }
        })
    });

    seeds.into_iter().min().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<i64>, Vec<Vec<Vec<i64>>>)) -> i64 {
    let mut min = i64::MAX;
    for w in input.0.chunks(2) {
        let seeds = (w[0]..(w[0] + w[1])).collect::<Vec<i64>>();
        min = min.min(part1(&(seeds, input.1.clone())));
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            )),
            35
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            )),
            46
        );
    }
}
