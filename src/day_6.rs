use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part_1() -> usize {
    let reader = BufReader::new(File::open("input/day_6.txt").unwrap());
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

pub fn part_2() -> usize {
    let reader = BufReader::new(File::open("input/day_6.txt").unwrap());
    let mut r = 0;
    let mut c = 0;
    let mut hs: std::collections::HashSet<(usize, usize, usize)> = std::collections::HashSet::new();
    let mut h: HashSet<(usize, usize)> = HashSet::new();
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
                r = row + 1;
                c = line.find('^').unwrap();
            }
            line.chars().collect()
        })
        .collect();
    let mut start_row = r;
    let mut start_col = c;
    grid.push(vec!['X'; grid[0].len()]);
    grid.insert(0, vec!['X'; grid[0].len()]);
    let mut curr_dir = 0;
    while grid[start_row][start_col] != 'X' {
        if h.insert((start_row, start_col)) {
            hs.insert((start_row, start_col, curr_dir));
        }
        let next_pos = grid[(start_row as i32 + DIRS[curr_dir].0) as usize]
            [(start_col as i32 + DIRS[curr_dir].1) as usize];
        if next_pos == '#' {
            curr_dir = (curr_dir + 1) % 4;
        }
        start_col = (start_col as i32 + DIRS[curr_dir].1) as usize;
        start_row = (start_row as i32 + DIRS[curr_dir].0) as usize;
    }
    hs.iter()
        .filter(|s| s.0 != r || s.1 != c)
        .map(|s| (s.0, s.1, s.2))
        .map(|(row, col, dir)| {
            h.clear();
            let mut g = grid.clone();
            g[row][col] = '#';
            start_row = r;
            start_col = c;
            curr_dir = 0;
            let mut hsx: HashSet<(usize, usize, usize)> = HashSet::new();
            while g[start_row][start_col] != 'X' {
                if h.insert((start_row, start_col)) {
                    hsx.insert((start_row, start_col, curr_dir));
                }
                let next_pos = g[(start_row as i32 + DIRS[curr_dir].0) as usize]
                    [(start_col as i32 + DIRS[curr_dir].1) as usize];
                if next_pos == '#' {
                    curr_dir = (curr_dir + 1) % 4;
                    continue;
                }
                start_row = (start_row as i32 + DIRS[curr_dir].0) as usize;
                start_col = (start_col as i32 + DIRS[curr_dir].1) as usize;

                if hsx.contains(&(start_row, start_col, curr_dir)) {
                    return 1;
                }
            }
            0
        })
        .sum()
}
