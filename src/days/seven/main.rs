use crate::days::Day;
use std::collections::HashMap;
use std::collections::HashSet;

type Point = (usize, usize);

type Grid = Vec<Vec<char>>;

fn is_split_destination(g: &Grid, y: usize, x: usize) -> bool {
    g[y][x] == '^'
}

fn emits_laser(p: char) -> bool {
    p == 'S' || p == '|'
}

fn part1(data: Vec<String>) -> u64 {
    let mut g: Grid = data.into_iter().map(|x| x.chars().collect()).collect();
    let mut splits: HashSet<Point> = HashSet::new();

    for y in 0..g.len() - 1 {
        for x in 0..g[0].len() {
            if emits_laser(g[y][x]) {
                if !is_split_destination(&g, y + 1, x) {
                    g[y + 1][x] = '|';
                } else {
                    // split left
                    if x != 0 {
                        g[y + 1][x - 1] = '|';
                        splits.insert((x, y + 1));
                    }

                    // split right
                    if x != g[0].len() - 1 {
                        g[y + 1][x + 1] = '|';
                        splits.insert((x, y + 1));
                    }
                }
            }
        }
    }

    splits.len() as u64
}

fn part2(data: Vec<String>) -> u64 {
    // 1. Static Grid (Read Only)
    let grid: Vec<Vec<char>> = data.into_iter().map(|x| x.chars().collect()).collect();
    let dim_y = grid.len();
    let dim_x = grid[0].len();

    let mut timeline_counts: HashMap<usize, u64> = HashMap::new();

    for x in 0..dim_x {
        if emits_laser(grid[0][x]) {
            *timeline_counts.entry(x).or_insert(0) += 1;
        }
    }

    for y in 0..dim_y - 1 {
        let mut next_counts: HashMap<usize, u64> = HashMap::new();

        for (&x, &count) in &timeline_counts {
            if !is_split_destination(&grid, y + 1, x) {
                *next_counts.entry(x).or_insert(0) += count;
            } else {
                if x > 0 {
                    *next_counts.entry(x - 1).or_insert(0) += count;
                }

                if x < dim_x - 1 {
                    *next_counts.entry(x + 1).or_insert(0) += count;
                }
            }
        }

        timeline_counts = next_counts;

        if timeline_counts.is_empty() {
            break;
        }
    }

    timeline_counts.values().sum()
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
