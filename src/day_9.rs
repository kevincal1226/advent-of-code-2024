use std::collections::HashSet;
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
    let mut bullshit: Vec<(char, usize, usize)> = reader
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(i, s)| {
            if i % 2 == 0 {
                (
                    (i / 2).to_string().chars().next().unwrap(),
                    s.to_digit(10).unwrap() as usize,
                    i / 2,
                )
            } else {
                ('.', s.to_digit(10).unwrap() as usize, i / 2)
            }
        })
        .collect();
    let mut discovered: HashSet<usize> = HashSet::new();
    let mut sum = 0;
    for mut i in (0..bullshit.len()).rev() {
        if bullshit[i].0 == '.' {
            continue;
        }
        if !discovered.insert(bullshit[i].2) {
            continue;
        }
        let mut replace_index = 0;
        let mut idx = i;
        for j in 0..i {
            if bullshit[j].0 != '.' || bullshit[j].1 < bullshit[i].1 {
                replace_index += bullshit[j].1;
                continue;
            }
            idx = j;
            break;
        }
        for s in replace_index..replace_index + bullshit[i].1 {
            sum += s * bullshit[i].2;
        }
        if idx != i {
            bullshit[idx].1 -= bullshit[i].1;
            bullshit[i].0 = '.';
            bullshit.insert(
                idx,
                (
                    bullshit[i].2.to_string().chars().next().unwrap(),
                    bullshit[i].1,
                    bullshit[i].2,
                ),
            );
        }
    }
    sum
}
