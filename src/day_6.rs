use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part_1(input: String) -> usize {
    let reader = BufReader::new(File::open(input).unwrap());
    let mut start_row = 0;
    let mut start_col = 0;
    let mut hs: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| {
            let mut res = "X".to_owned();
            res.push_str(&s);
            res.push('X');
            res
        })
        .enumerate()
        .map(|(row, line)| {
            if line.contains('^') {
                start_row = row + 1;
                start_col = line.find('^').unwrap();
            }
            line.chars().collect()
        })
        .collect();
    grid.push(vec!['X'; grid[0].len()]);
    grid.insert(0, vec!['X'; grid[0].len()]);
    let mut curr_dir = 0;
    while grid[start_row][start_col] != 'X' {
        hs.insert((start_row, start_col));
        let next_pos = grid[(start_row as i32 + DIRS[curr_dir].0) as usize]
            [(start_col as i32 + DIRS[curr_dir].1) as usize];
        if next_pos == '#' {
            curr_dir = (curr_dir + 1) % 4;
        }
        start_col = (start_col as i32 + DIRS[curr_dir].1) as usize;
        start_row = (start_row as i32 + DIRS[curr_dir].0) as usize;
    }
    hs.len()
}

pub fn part_2(input: String) -> usize {
    let reader = BufReader::new(File::open(input).unwrap());
    let mut start_row = 0;
    let mut start_col = 0;
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| {
            let mut res = "X".to_owned();
            res.push_str(&s);
            res.push('X');
            res
        })
        .enumerate()
        .map(|(row, s)| {
            if s.contains('^') {
                start_row = row + 1;
                start_col = s.find('^').unwrap();
            }
            s.chars().collect()
        })
        .collect();

    grid.insert(0, vec!['X'; grid[0].len()]);
    grid.push(vec!['X'; grid[0].len()]);
    grid.iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .filter(|s| *s.1 == '.')
                .map(|(col, _)| {
                    let mut g_clone = grid.clone();
                    g_clone[row][col] = '#';
                    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
                    let mut curr_dir = 0;
                    let mut curr_row = start_row;
                    let mut curr_col = start_col;
                    while g_clone[curr_row][curr_col] != 'X' {
                        if !visited.insert((curr_row, curr_col, curr_dir)) {
                            return 1;
                        }
                        while g_clone[(curr_row as i32 + DIRS[curr_dir].0) as usize]
                            [(curr_col as i32 + DIRS[curr_dir].1) as usize]
                            == '#'
                        {
                            curr_dir = (curr_dir + 1) % 4;
                        }
                        curr_row = (curr_row as i32 + DIRS[curr_dir].0) as usize;
                        curr_col = (curr_col as i32 + DIRS[curr_dir].1) as usize;
                    }
                    0
                })
                .sum::<usize>()
        })
        .sum()
}
