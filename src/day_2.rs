use std::io::BufRead;

pub fn part_1() -> usize {
    std::io::BufReader::new(std::fs::File::open("input/day_2.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|line| {
            let v: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            v.windows(3).all(|w| {
                (w[2] - w[1]) * (w[1] - w[0]) > 0
                    && w[2].abs_diff(w[1]) <= 3
                    && w[1].abs_diff(w[0]) <= 3
            })
        })
        .count()
}

pub fn part_2() -> usize {
    std::io::BufReader::new(std::fs::File::open("input/day_2.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|line| {
            let v: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            (0..v.len()).any(|i| {
                let mut v_clone = v.clone();
                v_clone.remove(i);
                v_clone.windows(3).all(|w| {
                    (w[2] - w[1]) * (w[1] - w[0]) > 0
                        && w[2].abs_diff(w[1]) <= 3
                        && w[1].abs_diff(w[0]) <= 3
                })
            })
        })
        .count()
}
