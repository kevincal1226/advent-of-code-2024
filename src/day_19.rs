use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn backtrack(answer: &mut str, designs: &[String]) -> bool {
    if answer.is_empty() {
        return true;
    }
    for a in designs.iter() {
        if let Some(new_answer) = answer.strip_suffix(a) {
            if backtrack(&mut new_answer.to_owned(), designs) {
                return true;
            }
        }
    }
    false
}

fn dp_deez_nuts(answer: &mut str, designs: &[String]) -> usize {
    let mut dp: Vec<usize> = vec![0; answer.len() + 1];
    dp[0] = 1;
    (1..dp.len()).for_each(|i| {
        designs.iter().for_each(|s| {
            if s.len() > i {
                return;
            }
            if answer[i - s.len()..i] == *s {
                dp[i] += dp[i - s.len()];
            }
        });
    });
    *dp.last().unwrap()
}

pub fn part_1(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap()).lines();

    let designs: Vec<String> = reader
        .next()
        .unwrap()
        .unwrap()
        .split_terminator(", ")
        .map(|e| e.to_owned())
        .collect();
    reader.next();
    let mut to_match: Vec<String> = reader.map(|e| e.unwrap()).collect();
    to_match
        .iter_mut()
        .map(|s| backtrack(s, &designs) as usize)
        .sum()
}

pub fn part_2(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap()).lines();

    let designs: Vec<String> = reader
        .next()
        .unwrap()
        .unwrap()
        .split_terminator(", ")
        .map(|e| e.to_owned())
        .collect();
    reader.next();
    let mut to_match: Vec<String> = reader.map(|e| e.unwrap()).collect();
    to_match.iter_mut().map(|s| dp_deez_nuts(s, &designs)).sum()
}
