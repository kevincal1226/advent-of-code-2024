use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let mut stoned: Vec<usize> = reader
        .lines()
        .flat_map(|line| {
            line.expect("Failed to read line")
                .split_whitespace() // Splits by spaces
                .map(|num| num.parse::<usize>().expect("Failed to parse number"))
                .collect::<Vec<usize>>() // Collect numbers into a Vec temporarily
        })
        .collect();

    (0..25).for_each(|_| {
        let mut i = 0;
        while i < stoned.len() {
            if stoned[i] == 0 {
                stoned[i] = 1;
            } else if stoned[i].to_string().len() % 2 == 0 {
                let pow = 10usize.pow((stoned[i].to_string().len() / 2) as u32);
                let new_v = stoned[i] % pow;
                stoned.insert(i + 1, new_v);
                stoned[i] /= pow;
                i += 1;
            } else {
                stoned[i] *= 2024;
            }
            i += 1;
        }
    });
    stoned.len()
}

pub fn part_2(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let mut stoned: Vec<usize> = reader
        .lines()
        .flat_map(|line| {
            line.expect("Failed to read line")
                .split_whitespace() // Splits by spaces
                .map(|num| num.parse::<usize>().expect("Failed to parse number"))
                .collect::<Vec<usize>>() // Collect numbers into a Vec temporarily
        })
        .collect();

    (0..75).for_each(|it| {
        println!("iteration: {}", it);
        let mut i = 0;
        while i < stoned.len() {
            if stoned[i] == 0 {
                stoned[i] = 1;
            } else if stoned[i].to_string().len() % 2 == 0 {
                let pow = 10usize.pow((stoned[i].to_string().len() / 2) as u32);
                let new_v = stoned[i] % pow;
                stoned.insert(i + 1, new_v);
                stoned[i] /= pow;
                i += 1;
            } else {
                stoned[i] *= 2024;
            }
            i += 1;
        }
    });
    stoned.len()
}
