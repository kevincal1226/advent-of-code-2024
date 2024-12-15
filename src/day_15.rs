use phf::phf_map;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DIRS: phf::Map<char, (i64, i64)> = phf_map! {
    '^' => (-1, 0),
    '>' => (0, 1),
    '<' => (0, -1),
    'v' => (1, 0),
};

fn dfs(row: i64, col: i64, row_change: i64, col_change: i64, grid: &mut Vec<Vec<char>>) -> bool {
    match grid[row as usize][col as usize] {
        '.' => true,
        '#' => false,
        'O' => {
            let dfs_res = dfs(
                row + row_change,
                col + col_change,
                row_change,
                col_change,
                grid,
            );
            if dfs_res {
                let temp = grid[row as usize][col as usize];
                grid[row as usize][col as usize] =
                    grid[(row + row_change) as usize][(col + col_change) as usize];
                grid[(row + row_change) as usize][(col + col_change) as usize] = temp;
                true
            } else {
                false
            }
        }
        _ => panic!("WTF"),
    }
}

pub fn part_1(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file.clone()).unwrap());
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| line.chars().collect())
        .collect();
    reader = BufReader::new(File::open(file.replace(".txt", "_part_2.txt")).unwrap());
    let instructions: Vec<char> = reader
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut bot_row = 0;
    let mut bot_col = 0;
    grid.iter().enumerate().for_each(|(row, v)| {
        v.iter()
            .enumerate()
            .filter(|(_, c)| **c == '@')
            .for_each(|(col, c)| {
                if *c == '@' {
                    bot_row = row as i64;
                    bot_col = col as i64;
                }
            });
    });
    instructions.iter().for_each(|inst| {
        if dfs(
            bot_row + DIRS[inst].0,
            bot_col + DIRS[inst].1,
            DIRS[inst].0,
            DIRS[inst].1,
            &mut grid,
        ) {
            grid[bot_row as usize][bot_col as usize] = '.';
            bot_row += DIRS[inst].0;
            bot_col += DIRS[inst].1;
            grid[bot_row as usize][bot_col as usize] = '@';
        }
    });
    grid.iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'O')
                .map(|(col, _)| 100 * row + col)
                .sum::<usize>()
        })
        .sum()
}

pub fn part_2(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file.clone()).unwrap());
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            line.chars()
                .flat_map(|s| {
                    if s == 'O' {
                        vec!['[', ']']
                    } else if s == '@' {
                        vec![s, '.']
                    } else {
                        vec![s, s]
                    }
                })
                .collect()
        })
        .collect();
    reader = BufReader::new(File::open(file.replace(".txt", "_part_2.txt")).unwrap());
    let instructions: Vec<char> = reader
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut bot_row = 0;
    let mut bot_col = 0;
    grid.iter().enumerate().for_each(|(row, v)| {
        v.iter()
            .enumerate()
            .filter(|(_, c)| **c == '@')
            .for_each(|(col, _)| {
                bot_row = row as i64;
                bot_col = col as i64;
            });
    });
    instructions.iter().enumerate().for_each(|(i, inst)| {
        let dfs_res = dfs(
            bot_row + DIRS[inst].0,
            bot_col + DIRS[inst].1,
            DIRS[inst].0,
            DIRS[inst].1,
            &mut grid,
        );
        if dfs_res {
            grid[bot_row as usize][bot_col as usize] = '.';
            bot_row += DIRS[inst].0;
            bot_col += DIRS[inst].1;
            grid[bot_row as usize][bot_col as usize] = '@';
        }
    });
    grid.iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == '[')
                .map(|(col, _)| 100 * row + col)
                .sum::<usize>()
        })
        .sum()
}
