use scan_fmt::scan_fmt;
use scanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let max_x: i64 = 101;
    let max_y: i64 = 103;
    let mut quads: [usize; 4] = [0, 0, 0, 0];
    reader.lines().map(|l| l.unwrap()).for_each(|s| {
        let mut px: i64 = 0;
        let mut py: i64 = 0;
        let mut vx: i64 = 0;
        let mut vy: i64 = 0;
        sscanf!(&s, "p={},{} v={},{}", px, py, vx, vy);
        if vx < 0 {
            vx = max_x + vx;
        }
        if vy < 0 {
            vy = max_y + vy;
        }
        vx *= 100;
        vy *= 100;
        vx += px;
        vy += py;
        let fin_x = vx % max_x;
        let fin_y = vy % max_y;
        if fin_x > max_x / 2 && fin_y > max_y / 2 {
            quads[0] += 1;
        } else if fin_x < max_x / 2 && fin_y > max_y / 2 {
            quads[1] += 1;
        } else if fin_x < max_x / 2 && fin_y < max_y / 2 {
            quads[2] += 1;
        } else if fin_x > max_x / 2 && fin_y < max_y / 2 {
            quads[3] += 1;
        }
    });
    quads.iter().product()
}

struct Bot {
    pub px: i64,
    pub py: i64,
    pub vx: i64,
    pub vy: i64,
}

pub fn part_2(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let max_x: i64 = 101;
    let max_y: i64 = 103;
    let mut drone_locs: HashSet<(i64, i64)> = HashSet::new();
    let mut drones: Vec<Bot> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let mut b: Bot = Bot {
                px: 0,
                py: 0,
                vx: 0,
                vy: 0,
            };
            if let Ok((px, py, vx, vy)) = scan_fmt!(&line, "p={},{} v={}, {}", i64, i64, i64, i64) {
                b.px = px;
                b.py = py;
                b.vx = vx;
                b.vy = vy;
            }
            if b.vx < 0 {
                b.vx = max_x + b.vx;
            }
            if b.vy < 0 {
                b.vy = max_y + b.vy;
            }
            b
        })
        .collect();
    (0..10403).for_each(|second| {
        drones.iter_mut().for_each(|d| {
            drone_locs.remove(&(d.px, d.py));
            d.px = (d.px + d.vx) % max_x;
            d.py = (d.py + d.vy) % max_y;
        });
        drones.iter().for_each(|d| {
            drone_locs.insert((d.px, d.py));
        });
        println!("Time: {}\n-------------------------", second + 1);
        (0..103).for_each(|i| {
            (0..101).for_each(|j| {
                if drone_locs.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            });
            println!("");
        });
    });
    7774
}
