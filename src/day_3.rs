use regex::Regex;
use std::io::BufRead;

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_3.txt").unwrap());
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            re.captures_iter(line.as_str())
                .map(|l| {
                    let first_num = l[1].parse::<usize>().unwrap();
                    let second_num = l[2].parse::<usize>().unwrap();
                    first_num * second_num
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part_2() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_3.txt").unwrap());
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();
    let do_pattern = r"do\(\)(.*?)don't\(\)";
    let do_re = Regex::new(do_pattern).unwrap();
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut res = "do()".to_owned();
            res.push_str(&l);
            res.push_str("don't()");
            res
        })
        .map(|line| {
            do_re
                .find_iter(&line)
                .map(|mat| mat.as_str())
                .flat_map(|mat| {
                    re.captures_iter(mat).map(|cap| {
                        let first_num = cap[1].parse::<usize>().unwrap();
                        let second_num = cap[2].parse::<usize>().unwrap();
                        first_num * second_num
                    })
                })
                .sum::<usize>()
        })
        .sum()
}
