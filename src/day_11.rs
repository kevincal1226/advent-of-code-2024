use cached::proc_macro::cached;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let mut stoned: Vec<usize> = reader
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace() // Splits by spaces
                .map(|num| num.parse::<usize>().unwrap())
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

fn get_stoners(val: usize) -> Vec<usize> {
    if val == 0 {
        return vec![1];
    } else if val.to_string().len() % 2 == 0 {
        let pow = 10usize.pow((val.to_string().len() / 2) as u32);
        return vec![val / pow, val % pow];
    } else {
        return vec![val * 2024];
    }
}

#[cached]
fn helper(val: usize, blink: usize) -> usize {
    if blink == 75 {
        return 1;
    }
    let vals = get_stoners(val);
    vals.iter().map(|s| helper(*s, blink + 1)).sum()
}

pub fn part_2(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let stoned: Vec<usize> = reader
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace() // Splits by spaces
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>() // Collect numbers into a Vec temporarily
        })
        .collect();

    stoned.iter().map(|x| helper(*x, 0)).sum()
}
