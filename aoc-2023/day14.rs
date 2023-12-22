use aoc_runner_derive::{aoc, aoc_generator};
use array2d::Array2D;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash, Clone)]
struct Platform {
    dish: Vec<Vec<char>>
}

impl Platform {
    fn move_north(&self) -> Self {
        Platform {
            dish: Array2D::from_columns(&Platform {
                dish: Array2D::from_rows(&self.dish).unwrap().as_columns()
            }.move_west().dish).unwrap().as_rows()
        }
    }

    fn move_west(&self) -> Self {
        let mut string_dish = self.dish.iter().map(|l| l.iter().collect::<String>()).collect::<Vec<String>>();
        for string in string_dish.iter_mut() {
            while string.contains(".O") {
                *string = string.replace(".O", "O.");
            }
        }
        
        Platform {
            dish: string_dish.iter().map(|l| l.chars().collect()).collect()
        }
    }

    fn move_south(&self) -> Self {
        Platform {
            dish: Array2D::from_columns(&Platform {
                dish: Array2D::from_rows(&self.dish).unwrap().as_columns()
            }.move_east().dish).unwrap().as_rows()
        }
    }

    fn move_east(&self) -> Self {
        let mut string_dish = self.dish.iter().map(|l| l.iter().collect::<String>()).collect::<Vec<String>>();
        for string in string_dish.iter_mut() {
            while string.contains("O.") {
                *string = string.replace("O.", ".O");
            }
        }
        
        Platform {
            dish: string_dish.iter().map(|l| l.chars().collect()).collect()
        }
    }

    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Platform {
    Platform {
        dish: input.lines().map(|l| l.chars().collect()).collect()
    }
}

fn count(input: &Platform) -> usize {
    let mut total = 0;
    let len = input.dish.len();
    for (i, row) in input.dish.iter().enumerate() {
        for c in row {
            if *c == 'O' {
                total += len - i;
            }
        }
    }
    total
}

#[aoc(day14, part1)]
fn part1(input: &Platform) -> usize {
    count(&input.move_north())
}

#[aoc(day14, part2)]
fn part2(input: &Platform) -> usize {
    let mut cloned: Platform = input.clone();
    let mut cache = HashMap::new();
    for i in 1..1000000000 {
        cloned = cloned.move_north().move_west().move_south().move_east();
        if let Some(prior) = cache.insert(cloned.calculate_hash(), i) {
            if (1000000000 - i) % (i - prior) == 0 {
                break;
            }
        }
    }

    count(&cloned)
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT1)), 64);
    }
}