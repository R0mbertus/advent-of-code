use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day25)]
fn parse(input: &str) -> usize {
    let edges_remove = [("tvj", "cvx"), ("fsv", "spx"), ("kdk", "nct")];
    let mut nodes = HashMap::<&str, Vec<&str>>::new();

    let _edges = input
        .lines()
        .flat_map(|l| {
            let (node, connected) = l.split_once(": ").unwrap();

            connected
                .split(' ')
                .filter_map(|n| {
                    if !edges_remove.contains(&(n, node)) && !edges_remove.contains(&(node, n)) {
                        nodes
                            .entry(<&str>::clone(&n))
                            .or_default()
                            .push(<&str>::clone(&node));
                        nodes
                            .entry(<&str>::clone(&node))
                            .or_default()
                            .push(<&str>::clone(&n));
                        Some((node, n))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(&str, &str)>>()
        })
        .collect::<Vec<(&str, &str)>>();

    // let mut dot = String::from("graph {\n");
    // for (node, connected) in edges.iter() {
    //     dot += &format!("  {} -- {};\n", node, connected);
    // }
    // dot += "}";
    // std::fs::write("day25.dot", dot).unwrap();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from(["tvj"]);
    while let Some(node) = queue.pop_back() {
        if visited.insert(node) {
            queue.extend(nodes.get(node).unwrap_or(&Vec::new()).iter());
        }
    }

    visited.len() * (nodes.len() - visited.len())
}

#[aoc(day25, part1)]
fn part1(input: &usize) -> usize {
    *input
}

#[aoc(day25, part2)]
fn part2(_input: &usize) -> String {
    "Get 49 stars".to_string()
}
