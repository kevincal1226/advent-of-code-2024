use std::io::BufRead;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRECTIONS: [(i64, i64); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn search(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    row_dx: i64,
    col_dx: i64,
    count: usize,
) -> usize {
    if count == 4 {
        return 1;
    }
    if grid[(row as i64 + (row_dx * count as i64)) as usize]
        [(col as i64 + (col_dx * count as i64)) as usize]
        != XMAS[count]
    {
        return 0;
    }
    search(grid, row, col, row_dx, col_dx, count + 1)
}

fn search2(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let tl = grid[row - 1][col - 1];
    let tr = grid[row - 1][col + 1];
    let br = grid[row + 1][col + 1];
    let bl = grid[row + 1][col - 1];
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

pub fn part_1(input: String) -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
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
                .map(|(col, _)| {
                    DIRECTIONS
                        .iter()
                        .map(|s| search(&grid, row, col, s.0, s.1, 0))
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn part_2(input: String) -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
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
