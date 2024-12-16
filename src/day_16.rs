use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn dfs(
    row: usize,
    col: usize,
    dir: char,
    cost: usize,
    disc: &mut HashMap<(usize, usize), usize>,
    grid: &Vec<Vec<char>>,
) -> usize {
    if *disc.entry((row, col)).or_insert(usize::MAX) < cost || grid[row][col] == '#' {
        return usize::MAX;
    }
    disc.insert((row, col), cost);
    if grid[row][col] == 'E' {
        return cost;
    }
    match dir {
        'n' => *[
            dfs(row - 1, col, 'n', cost + 1, disc, grid),
            dfs(row, col - 1, 'w', cost + 1001, disc, grid),
            dfs(row, col + 1, 'e', cost + 1001, disc, grid),
        ]
        .iter()
        .min()
        .unwrap(),
        'e' => *[
            dfs(row, col + 1, 'e', cost + 1, disc, grid),
            dfs(row - 1, col, 'n', cost + 1001, disc, grid),
            dfs(row + 1, col, 's', cost + 1001, disc, grid),
        ]
        .iter()
        .min()
        .unwrap(),
        's' => *[
            dfs(row + 1, col, 's', cost + 1, disc, grid),
            dfs(row, col - 1, 'w', cost + 1001, disc, grid),
            dfs(row, col + 1, 'e', cost + 1001, disc, grid),
        ]
        .iter()
        .min()
        .unwrap(),
        'w' => *[
            dfs(row, col - 1, 'w', cost + 1, disc, grid),
            dfs(row - 1, col, 'n', cost + 1001, disc, grid),
            dfs(row + 1, col, 's', cost + 1001, disc, grid),
        ]
        .iter()
        .min()
        .unwrap(),
        _ => panic!("wtf"),
    }
}

pub fn part_1(file: String) -> usize {
    let grid: Vec<Vec<char>> = BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect())
        .collect();
    let row = grid.len() - 2;
    let col = 1;
    let mut discovered: HashMap<(usize, usize), usize> = HashMap::new();
    dfs(row, col, 'e', 0, &mut discovered, &grid)
}

pub fn part_2(file: String) -> usize {
    let grid: Vec<Vec<char>> = BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect())
        .collect();
    let row = grid.len() - 2;
    let col = 1;
    let mut discovered: HashMap<(usize, usize), usize> = HashMap::new();
    let best_path = dfs(row, col, 'e', 0, &mut discovered, &grid);
    best_path
}
