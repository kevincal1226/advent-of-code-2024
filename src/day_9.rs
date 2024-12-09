use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap()).lines();
    let mut ids: Vec<usize> = Vec::new();
    let bullshit: Vec<char> = reader
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .enumerate()
        .flat_map(|(i, s)| {
            if i % 2 == 0 {
                ids.push(s.to_digit(10).unwrap() as usize);
                "x".repeat(s.to_digit(10).unwrap() as usize)
                    .chars()
                    .collect::<Vec<char>>()
            } else {
                ".".repeat(s.to_digit(10).unwrap() as usize)
                    .chars()
                    .collect()
            }
        })
        .collect();
    let mut sum = 0;
    sum
}

pub fn part_2(file: String) -> usize {
    0
}
