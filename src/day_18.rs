use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(file: String) -> usize {
    0
    //let mut reader = BufReader::new(File::open(file).unwrap()).lines();
    //let mut grid: Vec<Vec<bool>> = vec![vec![true; 73]; 73];
    //(0..1024).for_each(|_| {
    //    let binding = reader.next().unwrap().unwrap();
    //    let mut x = binding.split_terminator(",");
    //    let (row, col) = (
    //        x.next().unwrap().parse::<usize>().unwrap(),
    //        x.next().unwrap().parse::<usize>().unwrap(),
    //    );
    //    grid[row + 1][col + 1] = false;
    //});
    //get_dicks_path(&grid).0
}

fn get_dicks_path(grid: &Vec<Vec<bool>>) -> (usize, HashSet<(usize, usize)>) {
    let mut adjlist: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut innies: HashSet<(usize, usize)> = HashSet::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    grid.iter().enumerate().for_each(|(row, v)| {
        v.iter().enumerate().for_each(|(col, b)| {
            adjlist.insert((row, col), vec![]);
            if !b || row == 0 || col == 0 || row == grid.len() - 1 || col == grid.len() - 1 {
                return;
            }
            innies.insert((row, col));
            distances.insert((row, col), usize::MAX);
            prev.insert((row, col), (0, 0));
            if grid[row - 1][col] {
                adjlist.get_mut(&(row, col)).unwrap().push((row - 1, col));
            }
            if grid[row + 1][col] {
                adjlist.get_mut(&(row, col)).unwrap().push((row + 1, col));
            }
            if grid[row][col - 1] {
                adjlist.get_mut(&(row, col)).unwrap().push((row, col - 1));
            }
            if grid[row][col + 1] {
                adjlist.get_mut(&(row, col)).unwrap().push((row, col + 1));
            }
        })
    });

    distances.entry((1, 1)).and_modify(|e| *e = 0);
    while !innies.is_empty() {
        let (min_vertex, dist) = distances
            .iter_mut()
            .filter(|(e, _)| innies.contains(e))
            .min_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
            .unwrap();
        innies.remove(min_vertex);

        let m = *min_vertex;

        let d = *dist;

        adjlist.get(min_vertex).unwrap().iter().for_each(|e| {
            if !innies.contains(e) {
                return;
            }
            let alt = d + 1;
            if alt < *distances.get(e).unwrap() {
                distances.entry(*e).and_modify(|e| *e = alt);
                prev.entry(*e).and_modify(|x| *x = m);
            }
        });
    }
    let mut path: HashSet<(usize, usize)> = HashSet::new();
    let mut r = 7;
    let mut c = 7;
    while *prev.get(&(r, c)).unwrap() != (0, 0) {
        path.insert((r, c));
        (r, c) = *prev.get(&(r, c)).unwrap();
    }
    (*distances.get(&(7, 7)).unwrap(), path)
}

fn bfs(row: usize, col: usize, grid: &Vec<Vec<bool>>, disco: &mut HashSet<(usize, usize)>) -> bool {
    if row == 0
        || col == 0
        || row == grid.len() - 1
        || col == grid.len() - 1
        || !grid[row][col]
        || !disco.insert((row, col))
    {
        return false;
    }
    if row == grid.len() - 2 && col == grid.len() - 2 {
        return true;
    }

    bfs(row - 1, col, grid, disco)
        || bfs(row + 1, col, grid, disco)
        || bfs(row, col - 1, grid, disco)
        || bfs(row, col + 1, grid, disco)
}

pub fn part_2(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap());
    let mut grid: Vec<Vec<bool>> = vec![vec![true; 73]; 73];
    let mut res = (0, 0);
    reader.lines().for_each(|line| {
        if res != (0, 0) {
            return;
        }
        let binding = line.unwrap();
        let mut x = binding.split_terminator(",");
        let (col, row) = (
            x.next().unwrap().parse::<usize>().unwrap(),
            x.next().unwrap().parse::<usize>().unwrap(),
        );
        grid[row + 1][col + 1] = false;
        let mut disco: HashSet<_> = HashSet::new();
        if !bfs(1, 1, &grid, &mut disco) {
            res = (col, row);
        }
    });
    println!("{res:?}");
    0
}
