use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::slice::Iter;

fn backtrack_1(sum: usize, nums: &[usize], index: usize, curr_sum: usize) -> bool {
    if sum == curr_sum && index == nums.len() {
        return true;
    } else if curr_sum > sum || index >= nums.len() {
        return false;
    } else {
        return (backtrack_1(sum, nums, index + 1, curr_sum + nums[index])
            || backtrack_1(sum, nums, index + 1, curr_sum * nums[index]));
    }
}

fn backtrack_2(sum: usize, nums: &[usize], index: usize, curr_sum: usize) -> bool {
    if sum == curr_sum && index == nums.len() {
        return true;
    } else if curr_sum > sum || index >= nums.len() {
        return false;
    } else {
        let s = nums[index].to_string();
        return (backtrack_2(sum, nums, index + 1, curr_sum + nums[index])
            || backtrack_2(sum, nums, index + 1, curr_sum * nums[index]))
            || backtrack_2(
                sum,
                nums,
                index + 1,
                curr_sum * 10usize.pow(s.len() as u32) + nums[index],
            );
    }
}

pub fn part_1(input: String) -> usize {
    let reader = BufReader::new(File::open(input).unwrap());
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            let mut x = s.split_terminator(":");
            (
                x.next().unwrap().parse::<usize>().unwrap(),
                x.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(sum, nums)| match backtrack_1(sum, &nums, 1, nums[0]) {
            true => sum,
            false => 0,
        })
        .sum()
}

pub fn part_2(input: String) -> usize {
    let reader = BufReader::new(File::open(input).unwrap());
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            let mut x = s.split_terminator(":");
            (
                x.next().unwrap().parse::<usize>().unwrap(),
                x.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(sum, nums)| match backtrack_2(sum, &nums, 1, nums[0]) {
            true => sum,
            false => 0,
        })
        .sum()
}
