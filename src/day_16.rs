use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq)]
struct Dicky {
    dist: usize,
    row: usize,
    col: usize,
    prev: (usize, usize),
    path: HashSet<(usize, usize)>,
}

impl Ord for Dicky {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for Dicky {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_1(file: String) -> usize {
    let grid: Vec<Vec<char>> = BufReader::new(File::open(file.clone()).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect())
        .collect();
    let mut adjlist: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    (0..grid.len()).for_each(|row| {
        (0..grid[0].len()).for_each(|col| {
            adjlist.insert((row, col), vec![]);
            distances.insert((row, col), usize::MAX);
            if grid[row][col] == '#' {
                return;
            }
            if grid[row - 1][col] != '#' {
                adjlist.get_mut(&(row, col)).unwrap().push((row - 1, col));
            }
            if grid[row + 1][col] != '#' {
                adjlist.get_mut(&(row, col)).unwrap().push((row + 1, col));
            }
            if grid[row][col - 1] != '#' {
                adjlist.get_mut(&(row, col)).unwrap().push((row, col - 1));
            }
            if grid[row][col + 1] != '#' {
                adjlist.get_mut(&(row, col)).unwrap().push((row, col + 1));
            }
        })
    });
    let row = grid.len() - 2;
    let col = 1;
    let mut pq = BinaryHeap::new();
    pq.push(Dicky {
        dist: 0,
        row,
        col,
        prev: (row, row - 1),
        path: HashSet::from_iter([(row, col)]),
    });

    while let Some(temp) = pq.pop() {
        if grid[temp.row][temp.col] == 'E' {
            return temp.dist;
        }
        if temp.dist > distances[&(temp.row, temp.col)] {
            continue;
        }

        adjlist[&(temp.row, temp.col)].iter().for_each(|e| {
            let mut next = Dicky {
                dist: temp.dist + 1,
                row: e.0,
                col: e.1,
                prev: (temp.row, temp.col),
                path: temp.path.clone(),
            };
            next.path.insert((temp.row, temp.col));
            if next.row != temp.prev.0 && next.col != temp.prev.1 {
                next.dist += 1000;
            }

            if next.dist <= distances[&(next.row, next.col)] {
                pq.push(next.clone());
                *distances.get_mut(&(next.row, next.col)).unwrap() = next.dist;
            }
        });
    }
    0
}

pub fn part_2(file: String) -> usize {
    let grid: Vec<Vec<char>> = BufReader::new(File::open(file.clone()).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect())
        .collect();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut innies: HashSet<(usize, usize)> = HashSet::new();

    grid.iter().enumerate().for_each(|(row, v)| {
        v.iter().enumerate().for_each(|(col, x)| {
            if *x == '#' {
                return;
            }
            distances.insert((row, col), usize::MAX);
            prev.insert((row, col), vec![]);
            innies.insert((row, col));
        });
    });

    let row = grid.len() - 2;
    let col = 1;
    *distances.get_mut(&(row, col)).unwrap() = 0;
    prev.get_mut(&(row, col)).unwrap().push((row, col - 1));

    while !innies.is_empty() {
        let u = *innies
            .iter()
            .min_by(|x, y| distances[*x].cmp(&distances[y]))
            .unwrap();
        innies.remove(&u);
        if innies.contains(&(u.0 - 1, u.1)) {
            let mut alt = distances[&u] + 1;
            if u.1 != prev[&u].last().unwrap().1 {
                alt += 1000;
            }
            if alt < distances[&(u.0 - 1, u.1)] {
                *distances.get_mut(&(u.0 - 1, u.1)).unwrap() = alt;
                prev.get_mut(&(u.0 - 1, u.1)).unwrap().push(u);
            }
        }
        if innies.contains(&(u.0 + 1, u.1)) {
            let mut alt = distances[&u] + 1;
            if u.1 != prev[&u].last().unwrap().1 {
                alt += 1000;
            }
            if alt < distances[&(u.0 + 1, u.1)] {
                *distances.get_mut(&(u.0 + 1, u.1)).unwrap() = alt;
                prev.get_mut(&(u.0 + 1, u.1)).unwrap().push(u);
            }
        }
        if innies.contains(&(u.0, u.1 - 1)) {
            let mut alt = distances[&u] + 1;
            if u.0 != prev[&u].last().unwrap().0 {
                alt += 1000;
            }
            if alt < distances[&(u.0, u.1 - 1)] {
                *distances.get_mut(&(u.0, u.1 - 1)).unwrap() = alt;
                prev.get_mut(&(u.0, u.1 - 1)).unwrap().push(u);
            }
        }
        if innies.contains(&(u.0, u.1 + 1)) {
            let mut alt = distances[&u] + 1;
            if u.0 != prev[&u].last().unwrap().0 {
                alt += 1000;
            }
            if alt < distances[&(u.0, u.1 + 1)] {
                *distances.get_mut(&(u.0, u.1 + 1)).unwrap() = alt;
                prev.get_mut(&(u.0, u.1 + 1)).unwrap().push(u);
            }
        }
    }
    prev.iter().for_each(|i| {
        println!("({} {}): {:?}", i.0 .0, i.0 .1, i.1);
    });

    HashSet::<(usize, usize)>::from_iter(prev[&(col, row)].clone()).len()
}
