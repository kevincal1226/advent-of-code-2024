use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn dfs(grid: &Vec<Vec<i64>>, row: i64, col: i64, prev: i64, edges: &mut HashSet<(i64, i64)>) {
    if row < 0 || row >= grid.len() as i64 || col < 0 || col >= grid[0].len() as i64 {
        return;
    }
    let e = grid[row as usize][col as usize];
    if prev + 1 != e {
        return;
    }
    if e == 9 {
        edges.insert((row, col));
        return;
    }
    dfs(grid, row - 1, col, e, edges);
    dfs(grid, row + 1, col, e, edges);
    dfs(grid, row, col - 1, e, edges);
    dfs(grid, row, col + 1, e, edges);
}

fn dfs2(grid: &Vec<Vec<i64>>, row: i64, col: i64, prev: i64, edges: &mut Vec<i64>) {
    if row < 0 || row >= grid.len() as i64 || col < 0 || col >= grid[0].len() as i64 {
        return;
    }
    let e = grid[row as usize][col as usize];
    if prev + 1 != e {
        return;
    }
    if e == 9 {
        edges.push(0);
        return;
    }
    dfs2(grid, row - 1, col, e, edges);
    dfs2(grid, row + 1, col, e, edges);
    dfs2(grid, row, col - 1, e, edges);
    dfs2(grid, row, col + 1, e, edges);
}
pub fn part_1(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let grid: Vec<Vec<i64>> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|l| l.chars().map(|e| e.to_digit(10).unwrap() as i64).collect())
        .collect();
    grid.iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter(|s| *s.1 == 0)
                .map(|(j, _)| {
                    let mut edges: HashSet<(i64, i64)> = HashSet::new();
                    dfs(&grid, i as i64 + 1, j as i64, 0, &mut edges);
                    dfs(&grid, i as i64 - 1, j as i64, 0, &mut edges);
                    dfs(&grid, i as i64, j as i64 + 1, 0, &mut edges);
                    dfs(&grid, i as i64, j as i64 - 1, 0, &mut edges);
                    edges.len()
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part_2(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let grid: Vec<Vec<i64>> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|l| l.chars().map(|e| e.to_digit(10).unwrap() as i64).collect())
        .collect();
    grid.iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter(|s| *s.1 == 0)
                .map(|(j, _)| {
                    let mut edges = vec![];
                    dfs2(&grid, i as i64 + 1, j as i64, 0, &mut edges);
                    dfs2(&grid, i as i64 - 1, j as i64, 0, &mut edges);
                    dfs2(&grid, i as i64, j as i64 + 1, 0, &mut edges);
                    dfs2(&grid, i as i64, j as i64 - 1, 0, &mut edges);
                    edges.len()
                })
                .sum::<usize>()
        })
        .sum()
}
