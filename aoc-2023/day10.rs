use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn get_valid_moves(pos: (i64, i64)) -> [char; 4] {
    match pos {
        (0, -1) => ['-', 'L', 'F', 'S'],
        (0, 1)  => ['-', 'J', '7', 'S'],
        (1, 0) => ['|', 'J', 'L', 'S'],
        (-1, 0)  => ['|', 'F', '7', 'S'],
        _ => unreachable!()
    }
}

fn moves_to_piece(pos: (i64, i64), moves: Vec<(i64, i64)>) -> char {
    let relative = moves.into_iter().map(|(y, x)| (y - pos.0, x - pos.1)).collect::<Vec<(i64, i64)>>();

    if relative.contains(&(-1, 0)) {
        if relative.contains(&(1, 0)) {
            '|'
        } else if relative.contains(&(0, -1)) {
            'J'
        } else {
            'L'
        }
    } else if relative.contains(&(1, 0)) {
        if relative.contains(&(0, -1)) {
            '7'
        } else {
            'F'
        }
    } else {
        '-'
    }
}

fn get_neighbours(pipe: char, pos: (i64, i64), height: i64, width: i64) -> Vec<(i64, i64)> {
    match pipe {
        '|' => vec![(-1, 0), (1, 0)],
        '-' => vec![(0, -1), (0, 1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(0, -1), (1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        'S' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        _ => unreachable!()
    }.into_iter()
        .map(|(y, x)| (y + pos.0, x + pos.1))
        .filter(|(y, x)| {
            let valid_val = |val, dim| val >= &0 && val < dim;
            valid_val(y, &height) && valid_val(x, &width)
        })
        .collect_vec()
}

#[aoc_generator(day10)]
fn parse(input: &str) -> (Vec<Vec<bool>>, Vec<Vec<char>>) {
    let mut maze = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let (height, width) = (maze.len() as i64, maze[0].len() as i64);
    let mut visited = vec![vec![false; width as usize]; height as usize];
    let minus = (width % 2 != 0) as i64;
    let pos_animal = (input.find('S').unwrap() as i64 / height, (input.find('S').unwrap() as i64 % width) - minus);
    let mut current = pos_animal;

    loop {
        visited[current.0 as usize][current.1 as usize] = true;
        current = **get_neighbours(maze[current.0 as usize][current.1 as usize], current, height, width)
            .iter()
            .filter(|(y, x)| {
                get_valid_moves((y - current.0, x - current.1) ).iter().contains(&maze[*y as usize][*x as usize]) && !visited[*y as usize][*x as usize]
            })
            .collect_vec().get(0).unwrap_or(&&(-1, -1));
        if current == (-1, -1) {
            break;
        }
    }

    let animal_replacement = get_neighbours('S', pos_animal, height, width).into_iter().filter(|(y, x)| {
        get_valid_moves((y - pos_animal.0, x - pos_animal.1) ).iter().contains(&maze[*y as usize][*x as usize]) && visited[*y as usize][*x as usize]
    }).collect();
    maze[pos_animal.0 as usize][pos_animal.1 as usize] = moves_to_piece(pos_animal, animal_replacement);

    (visited, maze)
}

#[aoc(day10, part1)]
fn part1(input: &(Vec<Vec<bool>>, Vec<Vec<char>>)) -> usize {
    (input.0.clone().into_iter().flatten().filter(|p| *p == true).collect::<Vec<bool>>().len()) / 2
}

#[aoc(day10, part2)]
fn part2(input: &(Vec<Vec<bool>>, Vec<Vec<char>>)) -> usize {
    let height = input.0.len();
    let collide = |prev, current| (current == '-' || current == 'F' || current == '7' || (prev != 'F' && current == 'J') || (prev != '7' && current == 'L'));
    let mut total = 0;
    
    for (y, row) in input.0.clone().into_iter().enumerate() {
        for (x, elem) in row.into_iter().enumerate() {
            if elem {
                continue;
            }
            let mut ray_y = y + 1;
            let mut collisions = 0;
            let mut last_crossed = '.';
            while ray_y < height {
                if input.0[ray_y][x] && input.1[ray_y][x] != '|' {
                    if collide(last_crossed, input.1[ray_y][x]) {
                        collisions += 1;
                    }
                    last_crossed = input.1[ray_y][x];
                }
                ray_y += 1;
            }
            if collisions % 2 != 0 {
                total += 1;
            }
        }
    }

    total
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = 
".....
.S-7.
.|.|.
.L-J.
.....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 4);
    }

    const INPUT2: &str = 
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn part2_1_example() {
        assert_eq!(part2(&parse(INPUT2)), 8);
    }

    const INPUT3: &str = 
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part2_2_example() {
        assert_eq!(part2(&parse(INPUT3)), 10);
    }
}  
