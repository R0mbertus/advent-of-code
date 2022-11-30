use std::fs::read_to_string;
use std::cmp::max;

fn get_visibility(heights: Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut visibility = vec![vec![false; heights[0].len()]; heights.len()];
    let (height, width) = (heights.len() - 1, heights[0].len() - 1);

    let mut last_visible = (0, 0);
    for line in 0..=height {
        for tree in 0..=width {
            if line == 0 || line == height || tree == 0 || heights[last_visible.0][last_visible.1] < heights[line][tree] {
                visibility[line][tree] = true;
                last_visible = (line, tree);
            }
        }
        for tree in (0..=width).rev() {
            if line == 0 || line == height || tree == width || heights[last_visible.0][last_visible.1] < heights[line][tree] {
                visibility[line][tree] = true;
                last_visible = (line, tree);
            }
        }
    }
    for tree in 0..=width {
        for line in 0..=height {
            if tree == 0 || tree == width || line == 0 || heights[last_visible.0][last_visible.1] < heights[line][tree] {
                visibility[line][tree] = true;
                last_visible = (line, tree);
            }
        }
        for line in (0..=height).rev() {
            if tree == 0 || tree == width || line == height || heights[last_visible.0][last_visible.1] < heights[line][tree] {
                visibility[line][tree] = true;
                last_visible = (line, tree);
            }
        }
    }
    visibility
}

fn get_most_scenic(heights: Vec<Vec<u8>>) -> usize {
    let (height, width) = (heights.len(), heights[0].len());
    let mut max_scenic = 0;
    for line in 0..height {
        for tree in 0..width {
            let current_height = heights[line][tree];
            let mut last_op = 0;
            let west = (0..tree).rev()
                .take_while(|t| if heights[line][*t] < current_height {true} else {last_op = 1; false})
                .count() + last_op;

            last_op = 0;
            let east = ((tree+1)..width)
                .take_while(|t| if heights[line][*t] < current_height {true} else {last_op = 1; false})
                .count() + last_op;

            last_op = 0;
            let north = (0..line).rev()
                .take_while(|t| if heights[*t][tree] < current_height {true} else {last_op = 1; false})
                .count() + last_op;

            last_op = 0;
            let south = ((line+1)..height)
                .take_while(|t| if heights[*t][tree] < current_height {true} else {last_op = 1; false})
                .count() + last_op;
            
            max_scenic = max(max_scenic, west * east * north * south)
        }
    }
    max_scenic
}

fn parse_height_trees(forest: &str) -> Vec<Vec<u8>> {
    forest.lines()
        .map(|l| l.as_bytes().to_vec()
            .into_iter()
            .map(|t| t - '0' as u8)
            .collect()
        ).collect::<Vec<Vec<u8>>>()
}

pub fn solve1() {
    let forest = read_to_string("src/days/input/8.txt")
        .unwrap();

    let visible_trees_count = get_visibility(parse_height_trees(forest.as_str()))
        .iter()
        .map(|r| r.iter()
            .map(|t| if *t {1} else {0}).sum::<usize>())
        .sum::<usize>();
    
    println!("  Part 1: {}", visible_trees_count);
}

pub fn solve2() {
    let forest = read_to_string("src/days/input/8.txt")
        .unwrap();
    
    println!("  Part 2: {}", get_most_scenic(parse_height_trees(forest.as_str())));
}
