use std::io::BufRead;

pub fn part_1(input: String) -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
    let mut lhs: Vec<usize> = Vec::new();
    let mut rhs: Vec<usize> = Vec::new();
    reader.lines().map(|l| l.unwrap()).for_each(|line| {
        let mut sw = line.split_whitespace();
        lhs.push(sw.next().unwrap().parse::<usize>().unwrap());
        rhs.push(sw.next().unwrap().parse::<usize>().unwrap());
    });
    lhs.sort();
    rhs.sort();
    lhs.iter()
        .zip(rhs)
        .map(|(i, j)| i.abs_diff(j))
        .sum::<usize>()
}

pub fn part_2(input: String) -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
    let mut lhs: Vec<usize> = Vec::new();
    let mut rhs: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    reader.lines().map(|l| l.unwrap()).for_each(|line| {
        let mut sw = line.split_whitespace();
        lhs.push(sw.next().unwrap().parse::<usize>().unwrap());
        *rhs.entry(sw.next().unwrap().parse::<usize>().unwrap())
            .or_default() += 1;
    });
    lhs.iter()
        .map(|elem| *elem * *rhs.entry(*elem).or_default())
        .sum()
}
