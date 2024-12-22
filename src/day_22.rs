use cached::proc_macro::cached;
use std::collections::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PRUNE: i64 = 16777216;

#[cached]
fn helper(secret: i64, i: usize) -> i64 {
    if i == 2000 {
        return secret;
    }
    helper(get_secret(secret), i + 1)
}

#[cached]
fn get_secret(mut secret: i64) -> i64 {
    secret ^= secret * 64;
    secret %= PRUNE;
    secret ^= secret / 32;
    secret %= PRUNE;
    secret ^= secret * 2048;
    secret %= PRUNE;
    secret
}

pub fn part_1(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    reader
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .map(|secret| helper(secret, 0) as usize)
        .sum()
}

pub fn part_2(file: String) -> usize {
    let mut best: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .for_each(|i| {
            let mut prev = i;
            let mut curr = i;

            let changes = (0..2000)
                .map(|_| {
                    curr = get_secret(prev);
                    let diff = curr % 10 - prev % 10;
                    prev = curr;
                    (curr % 10, diff)
                })
                .collect::<Vec<(i64, i64)>>();

            let mut seen: HashSet<(i64, i64, i64, i64)> = HashSet::new();

            changes.windows(4).for_each(|win| {
                if seen.insert((win[0].1, win[1].1, win[2].1, win[3].1)) {
                    best.entry((win[0].1, win[1].1, win[2].1, win[3].1))
                        .and_modify(|e| *e += win[3].0)
                        .or_insert(win[3].0);
                }
            });
        });
    let res = best.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    *res.1 as usize
}
