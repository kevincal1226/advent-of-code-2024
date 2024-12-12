use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(input: String) -> usize {
    let grid: Vec<Vec<char>> = BufReader::new(File::open(input).unwrap())
        .lines()
        .map(|s| s.unwrap())
        .map(|l| l.chars().collect())
        .collect();
    let mut disc: HashSet<(i64, i64)> = HashSet::new();
    (0..grid.len())
        .map(|r| r as i64)
        .map(|row| {
            (0..grid[0].len())
                .map(|c| c as i64)
                .map(|col| {
                    let mut stack: Vec<(i64, i64)> = Vec::new();
                    let mut area = 0;
                    let mut perim = 0;
                    let start_char = grid[row as usize][col as usize];
                    stack.push((row, col));
                    while let Some(temp) = stack.pop() {
                        if temp.0 < 0
                            || temp.0 >= grid.len() as i64
                            || temp.1 < 0
                            || temp.1 >= grid[0].len() as i64
                            || grid[temp.0 as usize][temp.1 as usize] != start_char
                            || !disc.insert(temp)
                        {
                            continue;
                        }
                        area += 1;
                        if temp.0 - 1 < 0
                            || grid[temp.0 as usize - 1][temp.1 as usize] != start_char
                        {
                            perim += 1;
                        }
                        if temp.0 + 1 >= grid.len() as i64
                            || grid[temp.0 as usize + 1][temp.1 as usize] != start_char
                        {
                            perim += 1;
                        }
                        if temp.1 + 1 >= grid[0].len() as i64
                            || grid[temp.0 as usize][temp.1 as usize + 1] != start_char
                        {
                            perim += 1;
                        }
                        if temp.1 - 1 < 0
                            || grid[temp.0 as usize][temp.1 as usize - 1] != start_char
                        {
                            perim += 1;
                        }
                        stack.push((temp.0 + 1, temp.1));
                        stack.push((temp.0 - 1, temp.1));
                        stack.push((temp.0, temp.1 - 1));
                        stack.push((temp.0, temp.1 + 1));
                    }
                    area * perim
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part_2(input: String) -> usize {
    let mut grid: Vec<Vec<char>> = BufReader::new(File::open(input).unwrap())
        .lines()
        .map(|s| s.unwrap())
        .map(|s| {
            let mut res = ".".to_owned();
            res.push_str(&s);
            res.push('.');
            res
        })
        .map(|l| l.chars().collect())
        .collect();
    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);
    let mut disc: HashSet<(i64, i64)> = HashSet::new();
    (0..grid.len())
        .map(|r| r as i64)
        .map(|row| {
            (0..grid[0].len())
                .map(|c| c as i64)
                .filter(|e| grid[row as usize][*e as usize] != '.')
                .map(|col| {
                    let mut edged: HashSet<(i64, i64, i64)> = HashSet::new();
                    let mut stack: std::collections::VecDeque<(i64, i64)> =
                        std::collections::VecDeque::new();
                    let mut area = 0;
                    let mut perim = 0;
                    let start_char = grid[row as usize][col as usize];
                    stack.push_front((row, col));
                    while let Some(temp) = stack.pop_back() {
                        if grid[temp.0 as usize][temp.1 as usize] != start_char
                            || !disc.insert((temp.0, temp.1))
                        {
                            continue;
                        }
                        area += 1;
                        if grid[temp.0 as usize - 1][temp.1 as usize] != start_char {
                            perim += 1;
                        }
                        if grid[temp.0 as usize + 1][temp.1 as usize] != start_char {
                            perim += 1;
                        }
                        if grid[temp.0 as usize][temp.1 as usize + 1] != start_char {
                            perim += 1;
                        }
                        if grid[temp.0 as usize][temp.1 as usize - 1] != start_char {
                            perim += 1;
                        }
                        stack.push_back((temp.0 + 1, temp.1));
                        stack.push_back((temp.0 - 1, temp.1));
                        stack.push_back((temp.0, temp.1 - 1));
                        stack.push_back((temp.0, temp.1 + 1));
                    }
                    area * perim
                })
                .sum::<usize>()
        })
        .sum()
}
