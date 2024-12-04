use std::io::BufRead;

fn search(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut count = 0;
    if grid[row + 1][col] == 'M' && grid[row + 2][col] == 'A' && grid[row + 3][col] == 'S' {
        count += 1;
    }

    if grid[row - 1][col] == 'M' && grid[row - 2][col] == 'A' && grid[row - 3][col] == 'S' {
        count += 1;
    }

    if grid[row][col + 1] == 'M' && grid[row][col + 2] == 'A' && grid[row][col + 3] == 'S' {
        count += 1;
    }

    if grid[row][col - 1] == 'M' && grid[row][col - 2] == 'A' && grid[row][col - 3] == 'S' {
        count += 1;
    }

    if grid[row + 1][col + 1] == 'M'
        && grid[row + 2][col + 2] == 'A'
        && grid[row + 3][col + 3] == 'S'
    {
        count += 1;
    }
    if grid[row - 1][col + 1] == 'M'
        && grid[row - 2][col + 2] == 'A'
        && grid[row - 3][col + 3] == 'S'
    {
        count += 1;
    }
    if grid[row + 1][col - 1] == 'M'
        && grid[row + 2][col - 2] == 'A'
        && grid[row + 3][col - 3] == 'S'
    {
        count += 1;
    }
    if grid[row - 1][col - 1] == 'M'
        && grid[row - 2][col - 2] == 'A'
        && grid[row - 3][col - 3] == 'S'
    {
        count += 1;
    }
    count
}

fn search2(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let tl = grid[row - 1][col - 1];
    let tr = grid[row - 1][col + 1];
    let br = grid[row + 1][col + 1];
    let bl = grid[row + 1][col - 1];
    println!("{} {} {} {}", tl, tr, bl, br);
    if (tl != 'M' && tl != 'S')
        || (tr != 'M' && tr != 'S')
        || (br != 'M' && br != 'S')
        || (bl != 'M' && bl != 'S')
    {
        0
    } else {
        ((tl != br) && (tr != bl)) as usize
    }
}

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_4.txt").unwrap());
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut x = "Z".to_owned();
            x.push_str(&l);
            x.push('Z');
            x
        })
        .map(|line| line.chars().collect())
        .collect();

    (0..4).for_each(|_| grid.insert(0, vec!['Z'; grid[0].len()]));
    (0..4).for_each(|_| grid.push(vec!['Z'; grid[0].len()]));

    grid.iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'X')
                .map(|(col, _)| search(&grid, row, col))
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn part_2() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_4.txt").unwrap());
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut x = "Z".to_owned();
            x.push_str(&l);
            x.push('Z');
            x
        })
        .map(|line| line.chars().collect())
        .collect();

    (0..4).for_each(|_| grid.insert(0, vec!['Z'; grid[0].len()]));
    (0..4).for_each(|_| grid.push(vec!['Z'; grid[0].len()]));

    grid.iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'A')
                .map(|(col, _)| search2(&grid, row, col))
                .sum::<usize>()
        })
        .sum::<usize>()
}
