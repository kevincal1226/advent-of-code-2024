use multiset::HashMultiSet;
use std::collections::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

pub fn part_1(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap()).lines();
    let mut locks: HashMultiSet<Vec<usize>> = HashMultiSet::new();
    let mut keys: HashMultiSet<Vec<usize>> = HashMultiSet::new();
    while reader.next().is_some() {
        let s = reader.next().unwrap().unwrap();
        let mut freqs: Vec<usize> = vec![0; 5];
        (0..6).for_each(|_| {
            let v = reader.next().unwrap().unwrap();
            v.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(i, _)| {
                    freqs[i] += 1;
                });
        });
        if s.contains('.') {
            keys.insert(freqs);
        } else {
            locks.insert(freqs);
        }
    }

    locks
        .iter()
        .map(|l| {
            keys.iter()
                .filter(|x| zip(l.clone(), x.clone()).filter(|e| e.0 + e.1 >= 7).count() == 0)
                .count()
        })
        .sum()
}

pub fn part_2(file: String) -> usize {
    0
}
