use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap()).lines();
    let mut ids: Vec<(usize, usize)> = Vec::new();
    let bullshit: Vec<char> = reader
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .enumerate()
        .flat_map(|(i, s)| {
            if i % 2 == 0 {
                ids.push((i / 2, s.to_digit(10).unwrap() as usize));
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
    for i in 0..bullshit.len() {
        if ids.is_empty() {
            break;
        }
        if bullshit[i] == 'x' {
            sum += i * ids[0].0;
            ids[0].1 -= 1;
            if ids[0].1 == 0 {
                ids.remove(0);
            }
        } else {
            sum += i * ids.last().unwrap().0;
            ids.last_mut().unwrap().1 -= 1;
            if ids.last().unwrap().1 == 0 {
                ids.pop();
            }
        }
    }
    sum
}

pub fn part_2(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap()).lines();
    let mut ids: Vec<(usize, usize)> = Vec::new();
    let bullshit: Vec<(char, usize)> = reader
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(i, s)| {
            if i % 2 == 0 {
                ids.push((i / 2, s.to_digit(10).unwrap() as usize));
                ('x', s.to_digit(10).unwrap() as usize)
            } else {
                ('.', s.to_digit(10).unwrap() as usize)
            }
        })
        .collect();
    let mut sum = 0;

    sum
}
