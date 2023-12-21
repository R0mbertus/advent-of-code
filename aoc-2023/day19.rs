use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

struct Rule {
    input: char,
    ordering: char,
    treshold: usize,
    destination: String,
}

impl Rule {
    fn get_rating_val(&self, arg: &Rating) -> usize {
        match self.input {
            'x' => arg.x,
            'm' => arg.m,
            'a' => arg.a,
            's' => arg.s,
            _ => 0,
        }
    }

    fn compare(&self, arg: &Rating) -> Option<String> {
        let input = self.get_rating_val(arg);

        match self.ordering {
            '<' => (input < self.treshold).then(|| self.destination.clone()),
            '>' => (input > self.treshold).then(|| self.destination.clone()),
            '=' => Some(self.destination.clone()),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Rating>) {
    let (workflows, ratings) = input.split_once("\n\n").unwrap();
    (
        workflows.lines().fold(HashMap::new(), |mut acc, l| {
            let (key, conditions) = l.split_once('{').unwrap();
            let rules = conditions
                .trim_end_matches('}')
                .split(',')
                .map(|c| {
                    if c.len() > 4 {
                        Rule {
                            input: c.chars().nth(0).unwrap(),
                            ordering: c.chars().nth(1).unwrap(),
                            treshold: c[2..(c.find(':').unwrap() as usize)]
                                .parse::<usize>()
                                .unwrap(),
                            destination: c[c.find(':').unwrap() + 1..].to_string(),
                        }
                    } else {
                        Rule {
                            input: '0',
                            ordering: '=',
                            treshold: 0,
                            destination: c.to_string(),
                        }
                    }
                })
                .collect();
            acc.insert(key.to_string(), rules);
            acc
        }),
        ratings
            .lines()
            .map(|l| {
                let re = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)")
                    .unwrap()
                    .captures(l)
                    .unwrap();
                Rating {
                    x: re.get(1).unwrap().as_str().parse().unwrap(),
                    m: re.get(2).unwrap().as_str().parse().unwrap(),
                    a: re.get(3).unwrap().as_str().parse().unwrap(),
                    s: re.get(4).unwrap().as_str().parse().unwrap(),
                }
            })
            .collect(),
    )
}

#[aoc(day19, part1)]
fn part1(input: &(HashMap<String, Vec<Rule>>, Vec<Rating>)) -> usize {
    input
        .1
        .iter()
        .filter_map(|r| {
            let mut workflow = input.0.get("in").unwrap().iter();
            while let Some(rule) = workflow.next() {
                if let Some(res) = rule.compare(&r) {
                    match res.as_str() {
                        "A" => return Some(r.sum()),
                        "R" => return None,
                        _ => {
                            workflow = input.0.get(&res).unwrap().iter();
                        }
                    }
                }
            }
            unreachable!()
        })
        .sum()
}

#[aoc(day19, part2)]
fn part2(input: &(HashMap<String, Vec<Rule>>, Vec<Rating>)) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT1)), 19114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT1)), 167409079868000);
    }
}
