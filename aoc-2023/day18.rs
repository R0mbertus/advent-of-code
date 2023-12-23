use aoc_runner_derive::{aoc, aoc_generator};

struct Dig {
    dir: char,
    amount: usize,
    color: String,
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Dig> {
    input
        .lines()
        .filter_map(
            |l| match l.split_whitespace().collect::<Vec<&str>>().as_slice() {
                [d, v, c] => Some(Dig {
                    dir: d.chars().next().unwrap(),
                    amount: v.parse().unwrap(),
                    color: c.trim_matches(|c| ['(', '#', ')'].contains(&c)).to_string(),
                }),
                _ => None,
            },
        )
        .collect()
}

fn shoelace(points: Vec<(i64, i64)>) -> i64 {
    let (mut sum1, mut sum2) = (0, 0);

    for i in 0..(points.len() - 1) {
        sum1 += points[i].1 * points[i + 1].0;
        sum2 += points[i].0 * points[i + 1].1;
    }

    sum1 += points[points.len() - 1].0 * points[0].1;
    sum2 += points[points.len() - 1].1 * points[0].0;

    i64::abs(sum2 - sum1) / 2
}

fn pick(a: i64, b: i64) -> i64 {
    a + b / 2 + 1
}

#[aoc(day18, part1)]
fn part1(input: &[Dig]) -> i64 {
    let (_, points) = input
        .iter()
        .fold(((0, 0), vec![(0, 0)]), |((y, x), mut v), dig| {
            let point = match dig.dir {
                'U' => (y - dig.amount as i64, x),
                'L' => (y, x - dig.amount as i64),
                'D' => (y + dig.amount as i64, x),
                'R' => (y, x + dig.amount as i64),
                _ => unreachable!(),
            };
            v.push(point);
            (point, v)
        });

    pick(
        shoelace(points),
        input.iter().map(|d| d.amount).sum::<usize>() as i64,
    )
}

#[aoc(day18, part2)]
fn part2(input: &[Dig]) -> i64 {
    let hex_to_true = |hex: &String| {
        let (value, dir) = hex.split_at(5);
        (
            i64::from_str_radix(value, 16).unwrap(),
            dir.chars().next().unwrap(),
        )
    };

    let (_, perimeter, points) =
        input
            .iter()
            .fold(((0, 0), 0, vec![(0, 0)]), |((y, x), p, mut v), dig| {
                let (value, dir) = hex_to_true(&dig.color);
                let point = match dir {
                    '3' => (y - value, x),
                    '2' => (y, x - value),
                    '1' => (y + value, x),
                    '0' => (y, x + value),
                    _ => unreachable!(),
                };
                v.push(point);
                (point, p + value, v)
            });

    pick(shoelace(points), perimeter)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 62);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT1)), 952408144115);
    }
}
