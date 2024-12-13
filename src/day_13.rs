use nalgebra;
use scanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum MatrixType {
    One,
    Zero,
    Many,
}

fn rref(m: &mut nalgebra::Matrix2x3<f64>) -> MatrixType {
    m[(0, 1)] /= m[(0, 0)];
    m[(0, 2)] /= m[(0, 0)];
    m[(0, 0)] /= m[(0, 0)];
    let mut mult = m[(1, 0)];
    m[(1, 0)] -= mult * m[(0, 0)];
    m[(1, 1)] -= mult * m[(0, 1)];
    m[(1, 2)] -= mult * m[(0, 2)];
    if m[(1, 1)].trunc() == 0.0 {
        // multiple solns to mtx
        return MatrixType::Many;
    }
    m[(1, 2)] /= m[(1, 1)];
    m[(1, 1)] /= m[(1, 1)];
    mult = m[(0, 1)];
    m[(0, 1)] -= mult * m[(1, 1)];
    m[(0, 2)] -= mult * m[(1, 2)];
    if (m[(0, 2)].round().trunc() - m[(0, 2)]).abs() > 0.001
        || (m[(1, 2)].round().trunc() - m[(1, 2)]).abs() > 0.001
    {
        return MatrixType::Zero;
    }
    return MatrixType::One;
}

pub fn part_1(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap()).lines();
    let mut res = 0;
    while !reader.next().is_none() {
        let mut ax = 0.0;
        let mut ay = 0.0;
        let mut bx = 0.0;
        let mut by = 0.0;
        let mut tx = 0.0;
        let mut ty = 0.0;
        sscanf!(
            &reader.next().unwrap().unwrap(),
            "Button A: X+{}, Y+{}",
            ax,
            ay
        );
        sscanf!(
            &reader.next().unwrap().unwrap(),
            "Button B: X+{}, Y+{}",
            bx,
            by
        );
        sscanf!(
            &reader.next().unwrap().unwrap(),
            "Prize: X={}, Y={}",
            tx,
            ty
        );
        let mut mtx = nalgebra::Matrix2x3::new(ax, bx, tx, ay, by, ty);
        let t = rref(&mut mtx);
        if let MatrixType::One = t {
            res += mtx[(0, 2)].round().trunc() as usize * 3 + mtx[(1, 2)].round().trunc() as usize;
        }
    }
    res
}

pub fn part_2(file: String) -> usize {
    let mut reader = BufReader::new(File::open(file).unwrap()).lines();
    let mut res = 0;
    while !reader.next().is_none() {
        let mut ax = 0.0;
        let mut ay = 0.0;
        let mut bx = 0.0;
        let mut by = 0.0;
        let mut tx = 0.0;
        let mut ty = 0.0;
        sscanf!(
            &reader.next().unwrap().unwrap(),
            "Button A: X+{}, Y+{}",
            ax,
            ay
        );
        sscanf!(
            &reader.next().unwrap().unwrap(),
            "Button B: X+{}, Y+{}",
            bx,
            by
        );
        sscanf!(
            &reader.next().unwrap().unwrap(),
            "Prize: X={}, Y={}",
            tx,
            ty
        );
        tx += 10000000000000.0;
        ty += 10000000000000.0;
        let mut mtx = nalgebra::Matrix2x3::new(ax, bx, tx, ay, by, ty);
        let t = rref(&mut mtx);
        if let MatrixType::One = t {
            res += mtx[(0, 2)].round().trunc() as usize * 3 + mtx[(1, 2)].round().trunc() as usize;
        }
    }
    res
}
