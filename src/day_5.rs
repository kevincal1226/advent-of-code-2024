use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn is_valid(v: &[usize], ordering: &mut HashMap<usize, HashSet<usize>>) -> bool {
    (0..v.len()).all(|i| {
        (0..v.len()).all(|j| match i.cmp(&j) {
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Less => !ordering.entry(v[j]).or_default().contains(&v[i]),
            std::cmp::Ordering::Greater => !ordering.entry(v[i]).or_default().contains(&v[j]),
        })
    })
}

pub fn part_1(input: String) -> usize {
    let mut reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
    let mut ordering: HashMap<usize, HashSet<usize>> = HashMap::new();
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let line_iter = line.split_once("|").unwrap();
        ordering
            .entry(line_iter.0.parse::<usize>().unwrap())
            .or_default()
            .insert(line_iter.1.parse::<usize>().unwrap());
    }
    reader = std::io::BufReader::new(std::fs::File::open("input/day_5_part2.txt").unwrap());

    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            line.split_terminator(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .filter_map(|v: Vec<usize>| {
            let valid = is_valid(&v, &mut ordering);
            if valid {
                Some(v)
            } else {
                None
            }
        })
        .map(|v| v[v.len() / 2])
        .sum()
}

pub fn part_2(input: String) -> usize {
    let mut reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
    let mut ordering: HashMap<usize, HashSet<usize>> = HashMap::new();
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let line_iter = line.split_once("|").unwrap();
        ordering
            .entry(line_iter.0.parse::<usize>().unwrap())
            .or_default()
            .insert(line_iter.1.parse::<usize>().unwrap());
    }
    reader = std::io::BufReader::new(std::fs::File::open("input/day_5_part2.txt").unwrap());

    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            line.split_terminator(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .filter_map(|v: Vec<usize>| {
            let valid = is_valid(&v, &mut ordering);
            if valid {
                return None;
            }

            let mut new_v = v.clone();
            while !is_valid(&new_v, &mut ordering) {
                (0..new_v.len()).for_each(|i| {
                    (i + 1..new_v.len()).for_each(|j| {
                        if ordering.entry(new_v[j]).or_default().contains(&new_v[i]) {
                            new_v.swap(i, j);
                        }
                    })
                })
            }
            Some(new_v)
        })
        .map(|v| v[v.len() / 2])
        .sum()
}
