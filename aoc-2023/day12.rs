use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

type CacheKey = (Vec<char>, Vec<usize>, usize, usize, usize);

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (springs, nums) = l.split_once(' ').unwrap();
            (
                springs.chars().collect::<Vec<char>>(),
                nums.split(',').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn posibilities(
    cache: &mut HashMap<CacheKey, usize>,
    springs: &[char],
    spring_i: usize,
    nums: &[usize],
    nums_i: usize,
    count: usize,
) -> usize {
    let key = (springs.to_vec(), nums.to_vec(), spring_i, nums_i, count);
    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let result = if spring_i == springs.len() {
        (nums.len() == nums_i) as usize
    } else if springs[spring_i] == '#' {
        posibilities(cache, springs, spring_i + 1, nums, nums_i, count + 1)
    } else if springs[spring_i] == '.' || nums_i == nums.len() {
        if nums_i < nums.len() && count == nums[nums_i] {
            posibilities(cache, springs, spring_i + 1, nums, nums_i + 1, 0)
        } else if count == 0 {
            posibilities(cache, springs, spring_i + 1, nums, nums_i, 0)
        } else {
            0
        }
    } else {
        let broken = posibilities(cache, springs, spring_i + 1, nums, nums_i, count + 1);
        let dots = if count == nums[nums_i] {
            posibilities(cache, springs, spring_i + 1, nums, nums_i + 1, 0)
        } else if count == 0 {
            posibilities(cache, springs, spring_i + 1, nums, nums_i, 0)
        } else {
            0
        };
        broken + dots
    };
    cache.insert(key, result);
    result
}

#[aoc(day12, part1)]
fn part1(input: &[(Vec<char>, Vec<usize>)]) -> usize {
    let mut cache = HashMap::new();
    input
        .iter()
        .map(|l| {
            let mut springs = l.0.clone();
            springs.push('.');
            posibilities(&mut cache, &springs, 0, &l.1, 0, 0)
        })
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &[(Vec<char>, Vec<usize>)]) -> usize {
    let mut cache = HashMap::new();
    input
        .iter()
        .map(|l| {
            let mut springs = (0..5)
                .map(|_| l.0.iter().collect::<String>())
                .join("?")
                .chars()
                .collect::<Vec<char>>();
            springs.push('.');
            let nums = (0..5).flat_map(|_| l.1.clone()).collect::<Vec<usize>>();
            posibilities(&mut cache, &springs, 0, &nums, 0, 0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 525152);
    }
}
