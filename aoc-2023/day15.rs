use aoc_runner_derive::{aoc, aoc_generator};
use indexmap::IndexMap;

#[aoc_generator(day15, part1)]
fn parse1(input: &str) -> Vec<Vec<u8>> {
    input.split(',').map(|s| s.as_bytes().to_vec()).collect()
}

fn hash(v: &Vec<u8>) -> usize {
    v.iter().fold(0, |acc, c| ((acc + *c as usize) * 17) % 256)
}

#[aoc(day15, part1)]
fn part1(input: &[Vec<u8>]) -> usize {
    input.iter().map(hash).sum()
}

#[aoc_generator(day15, part2)]
fn parse2(input: &str) -> Vec<String> {
    input.split(',').map(str::to_string).collect()
}

#[aoc(day15, part2)]
fn part2(input: &[String]) -> usize {
    let mut hashmap = vec![IndexMap::new(); 256];
    for s in input {
        let box_entry = |key: &&str| hash(&key.as_bytes().to_vec());
        match s.trim_end_matches('-').split('=').collect::<Vec<&str>>().as_slice() {
            [key, value] => {
                hashmap[box_entry(key)].entry(*key).and_modify(|e| *e = value.parse::<usize>().unwrap()).or_insert(value.parse::<usize>().unwrap());
            },
            [key] => {hashmap[box_entry(key)].shift_remove_entry(*key);},
            _ => {}
        }
    }

    let mut total = 0;
    for (i, map) in hashmap.iter().enumerate() {
        for (j, elem) in map.values().enumerate() {
            total += elem * (i + 1) * (j + 1)
        }
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(INPUT1)), 1320);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(INPUT1)), 145);
    }
}