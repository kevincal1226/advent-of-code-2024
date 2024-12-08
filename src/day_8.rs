use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(input: String) -> usize {
    let reader = BufReader::new(File::open(input).unwrap());
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| line.chars().collect())
        .collect();
    let n = grid.len() as i64;
    let mut pairs: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    grid.iter().enumerate().for_each(|(row, v)| {
        v.iter()
            .enumerate()
            .filter(|(_, s)| **s != '.')
            .for_each(|(col, c)| {
                pairs.entry(*c).or_default().push((row as i64, col as i64));
            });
    });
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    pairs.iter().for_each(|(_, vec)| {
        vec.iter()
            .enumerate()
            .flat_map(|(idx, i)| {
                vec.iter()
                    .enumerate()
                    .filter(move |(j, _)| *j > idx)
                    .map(move |(_, j)| (i, j))
            })
            .for_each(|(p1, p2)| {
                let dx = p2.0 - p1.0;
                let dy = p2.1 - p1.1;
                if p1.0 - dx >= 0 && p1.1 - dy >= 0 && p1.0 - dx < n && p1.1 - dy < n {
                    antinodes.insert((p1.0 - dx, p1.1 - dy));
                }
                if p2.0 + dx >= 0 && p2.1 + dy >= 0 && p2.0 + dx < n && p2.1 + dy < n {
                    antinodes.insert((p2.0 + dx, p2.1 + dy));
                }
            })
    });
    antinodes.len()
}

pub fn part_2(input: String) -> usize {
    let reader = BufReader::new(File::open(input).unwrap());
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| line.chars().collect())
        .collect();
    let n = grid.len() as i64;
    let mut pairs: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    grid.iter().enumerate().for_each(|(row, v)| {
        v.iter()
            .enumerate()
            .filter(|(_, s)| **s != '.')
            .for_each(|(col, c)| {
                pairs.entry(*c).or_default().push((row as i64, col as i64));
            });
    });
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    pairs.iter().for_each(|(_, vec)| {
        vec.iter()
            .enumerate()
            .flat_map(|(idx, i)| {
                vec.iter()
                    .enumerate()
                    .filter(move |(j, _)| *j > idx)
                    .map(move |(_, j)| (i, j))
            })
            .for_each(|(p1, p2)| {
                let dx = p2.0 - p1.0;
                let dy = p2.1 - p1.1;
                let mut temp_x = p1.0;
                let mut temp_y = p1.1;
                while temp_x >= 0 && temp_y >= 0 && temp_x < n && temp_y < n {
                    antinodes.insert((temp_x, temp_y));
                    temp_x -= dx;
                    temp_y -= dy;
                }
                temp_x = p2.0;
                temp_y = p2.1;
                while temp_x >= 0 && temp_y >= 0 && temp_x < n && temp_y < n {
                    antinodes.insert((temp_x, temp_y));
                    temp_x += dx;
                    temp_y += dy;
                }
            })
    });
    antinodes.len()
}
