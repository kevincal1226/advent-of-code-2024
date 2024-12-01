use std::io::BufRead;

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_1.txt").unwrap());
    let mut lhs: Vec<usize> = Vec::new();
    let mut rhs: Vec<usize> = Vec::new();
    reader.lines().map(|l| l.unwrap()).for_each(|line| {
        let mut sw = line.split_whitespace();
        lhs.push(sw.next().unwrap().parse::<usize>().unwrap());
        rhs.push(sw.next().unwrap().parse::<usize>().unwrap());
    });
    lhs.sort();
    rhs.sort();
    (0..lhs.len()).map(|i| (rhs[i]).abs_diff(lhs[i])).sum()
}

pub fn part_2() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_1.txt").unwrap());
    let mut lhs: Vec<usize> = Vec::new();
    let mut rhs: Vec<usize> = Vec::new();
    reader.lines().map(|l| l.unwrap()).for_each(|line| {
        let mut sw = line.split_whitespace();
        lhs.push(sw.next().unwrap().parse::<usize>().unwrap());
        rhs.push(sw.next().unwrap().parse::<usize>().unwrap());
    });
    lhs.iter()
        .map(|elem| *elem * rhs.iter().filter(|e| **e == *elem).count())
        .sum()
}
