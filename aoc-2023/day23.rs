use std::{collections::HashSet, ops::Add};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos(i64, i64);

impl Add<char> for Pos {
    type Output = Pos;

    fn add(self, rhs: char) -> Self::Output {
        match rhs {
            '^' => Pos(self.0 - 1, self.1),
            '<' => Pos(self.0, self.1 - 1),
            'v' => Pos(self.0 + 1, self.1),
            '>' => Pos(self.0, self.1 + 1),
            _ => unreachable!()
        }
    }
}

impl Pos {
    fn successors(
        &self,
        trails: &Vec<Vec<char>>,
        sliding: bool
    ) -> Vec<Pos> {
        const SLIDES: [char; 4] = ['^', '<', 'v', '>'];

        if sliding {
            if let Some(slippery) = SLIDES.iter().position(|c| c == &trails[self.0 as usize][self.1 as usize]) {
                return vec![*self + SLIDES[slippery]];
            }
        }

        let mut successors = vec![];
        for p in [Pos(self.0 - 1, self.1), Pos(self.0, self.1 - 1), Pos(self.0 + 1, self.1), Pos(self.0, self.1 + 1)] {
            if p.0 >= 0 && p.1 >= 0 && p.0 < trails.len() as i64 && p.1 < trails[0].len() as i64 && trails[p.0 as usize][p.1 as usize] != '#' {
                successors.push(p);
            }
        }
        successors
    }
}

fn dfs(start: Pos, path: &mut HashSet<Pos>, trails: &Vec<Vec<char>>, sliding: bool) -> usize {
    if start == Pos(trails.len() as i64 - 1, trails[0].len() as i64 - 2) {
        return path.len()
    }
    let mut max = 0;
    for pos in start.successors(trails, sliding) {
        if path.insert(pos) {
            max = max.max(dfs(pos.clone(), path, trails, sliding));
            path.remove(&pos);
        }
    }
    max
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day23, part1)]
fn part1(input: &Vec<Vec<char>>) -> usize {
    dfs(Pos(0, 1), &mut HashSet::new(), input, true)
}

#[aoc(day23, part2)]
fn part2(input: &Vec<Vec<char>>) -> usize {
    dfs(Pos(0, 1), &mut HashSet::new(), input, false)
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 94);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT1)), 154);
    }
}