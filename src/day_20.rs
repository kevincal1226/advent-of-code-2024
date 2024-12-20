use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn dfs(
    grid: &[Vec<char>],
    start: (usize, usize),
    discovered: &mut HashSet<(usize, usize)>,
    distances: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let mut stack: std::collections::VecDeque<(usize, usize)> = std::collections::VecDeque::new();
    stack.push_back(start);
    let mut dist = 0;
    while !stack.is_empty() {
        let top = stack.pop_back().unwrap();
        if !discovered.insert(top) {
            continue;
        }

        distances.insert(top, dist);
        dist += 1;

        if grid[top.0][top.1] == 'E' {
            break;
        }

        if grid[top.0 - 1][top.1] != '#' {
            stack.push_back((top.0 - 1, top.1));
        }
        if grid[top.0 + 1][top.1] != '#' {
            stack.push_back((top.0 + 1, top.1));
        }
        if grid[top.0][top.1 - 1] != '#' {
            stack.push_back((top.0, top.1 - 1));
        }
        if grid[top.0][top.1 + 1] != '#' {
            stack.push_back((top.0, top.1 + 1));
        }
    }
    discovered.len() - 1
}

pub fn part_1(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect())
        .collect();

    let mut start_row = 0;
    let mut start_col = 0;
    let mut end_row = 0;
    let mut end_col = 0;
    grid.iter()
        .enumerate()
        .filter(|e| e.1.contains(&'S') || e.1.contains(&'E'))
        .for_each(|i| {
            if i.1.contains(&'S') {
                start_row = i.0;
                start_col = i.1.iter().position(|e| *e == 'S').unwrap();
            }
            if i.1.contains(&'E') {
                end_row = i.0;
                end_col = i.1.iter().position(|e| *e == 'E').unwrap();
            }
        });
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    dfs(
        &grid,
        (start_row, start_col),
        &mut HashSet::new(),
        &mut distances,
    );
    let mut pairs: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    distances.iter().for_each(|s| {
        distances.iter().for_each(|t| {
            let in_same_row_or_col = (s.0 .0 == t.0 .0) || (s.0 .1 == t.0 .1);
            let distance_between_is_exactly_2 =
                s.0 .0.abs_diff(t.0 .0) + s.0 .1.abs_diff(t.0 .1) == 2;
            let saves_ms = s.1.abs_diff(*t.1) - 2 >= 100;
            let in_between_is_block = grid[(s.0 .0 + t.0 .0) / 2][(s.0 .1 + t.0 .1) / 2] == '#';
            if in_same_row_or_col
                && distance_between_is_exactly_2
                && saves_ms
                && in_between_is_block
            {
                pairs.insert((std::cmp::min(*s.0, *t.0), std::cmp::max(*s.0, *t.0)));
            }
        })
    });
    pairs.len()
}

pub fn part_2(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect())
        .collect();

    let mut start_row = 0;
    let mut start_col = 0;
    let mut end_row = 0;
    let mut end_col = 0;
    grid.iter()
        .enumerate()
        .filter(|e| e.1.contains(&'S') || e.1.contains(&'E'))
        .for_each(|i| {
            if i.1.contains(&'S') {
                start_row = i.0;
                start_col = i.1.iter().position(|e| *e == 'S').unwrap();
            }
            if i.1.contains(&'E') {
                end_row = i.0;
                end_col = i.1.iter().position(|e| *e == 'E').unwrap();
            }
        });
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    dfs(
        &grid,
        (start_row, start_col),
        &mut HashSet::new(),
        &mut distances,
    );
    let mut pairs: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    distances.iter().for_each(|s| {
        distances.iter().for_each(|t| {
            let distance_between = s.0 .0.abs_diff(t.0 .0) + s.0 .1.abs_diff(t.0 .1);
            let distance_between_is_at_most_20 = distance_between <= 20;
            let saves_ms = s.1.abs_diff(*t.1) >= distance_between + 100;
            if distance_between_is_at_most_20 && saves_ms {
                pairs.insert((std::cmp::min(*s.0, *t.0), std::cmp::max(*s.0, *t.0)));
            }
        })
    });
    pairs.len()
}
