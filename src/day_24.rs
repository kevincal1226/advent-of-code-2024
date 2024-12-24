use std::collections::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

pub fn part_1(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let mut vals: HashMap<String, usize> = HashMap::new();
    let mut edges: HashMap<(String, String), HashSet<(String, String)>> = HashMap::new();
    reader.lines().map(|l| l.unwrap()).for_each(|l| {
        if l.is_empty() {
            return;
        }
        if l.contains(":") {
            let mut a = l.split_terminator(": ");
            vals.insert(
                a.next().unwrap().to_string(),
                a.next().unwrap().parse::<usize>().unwrap(),
            );
        } else {
            let mut a = l.split_whitespace();
            let l = a.next().unwrap();
            let op = a.next().unwrap();
            let r = a.next().unwrap();
            let junk = a.next().unwrap();
            let v = a.next().unwrap();
            edges
                .entry((l.to_string(), r.to_string()))
                .or_default()
                .insert((op.to_string(), v.to_string()));
            //edges.insert(
            //    (l.to_string(), r.to_string()),
            //    (op.to_string(), v.to_string()),
            //);
        }
    });
    loop {
        let mut to_remove: Vec<(String, String, String, String)> = Vec::new();
        edges.iter().for_each(|(e, hs)| {
            hs.iter().for_each(|(op, out)| {
                if !vals.contains_key(&e.0) || !vals.contains_key(&e.1) {
                    return;
                }
                let a = vals.get(&e.0).unwrap();
                let b = vals.get(&e.1).unwrap();
                match op.chars().next().unwrap() {
                    'A' => vals.insert(out.clone(), *a & *b),
                    'O' => vals.insert(out.clone(), *a | *b),
                    'X' => vals.insert(out.clone(), *a ^ *b),
                    _ => panic!("E"),
                };
                to_remove.push((e.0.clone(), e.1.clone(), op.clone(), out.clone()));
            });
        });
        if to_remove.is_empty() {
            break;
        }
        to_remove.iter().for_each(|e| {
            edges.entry((e.0.clone(), e.1.clone())).and_modify(|v| {
                v.remove(&(e.2.clone(), e.3.clone()));
            });
        });
    }
    vals.iter()
        .map(|v| {
            if !v.0.contains("z") || *v.1 == 0 {
                return 0;
            }
            let x = 2usize.pow(v.0.as_str()[1..3].parse::<u32>().unwrap());
            x
        })
        .sum()
}

pub fn part_2(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let mut vals: HashMap<String, usize> = HashMap::new();
    let mut edges_original: HashSet<(String, String, String, String)> = HashSet::new();
    reader.lines().map(|l| l.unwrap()).for_each(|l| {
        if l.is_empty() {
            return;
        }
        if l.contains(":") {
            let mut a = l.split_terminator(": ");
            vals.insert(
                a.next().unwrap().to_string(),
                a.next().unwrap().parse::<usize>().unwrap(),
            );
        } else {
            let mut a = l.split_whitespace();
            let l = a.next().unwrap();
            let op = a.next().unwrap();
            let r = a.next().unwrap();
            let junk = a.next().unwrap();
            let v = a.next().unwrap();
            edges_original.insert((l.to_string(), r.to_string(), op.to_string(), v.to_string()));
        }
    });
    for i in 0..edges_original.len() {
        for j in i + 1..edges_original.len() {
            for k in j + 1..edges_original.len() {
                for l in k + 1..edges_original.len() {
                    for m in l + 1..edges_original.len() {
                        for n in m + 1..edges_original.len() {
                            for o in n + 1..edges_original.len() {
                                for p in o + 1..edges_original.len() {
                                    let mut peen: Vec<(String, String, String, String)> =
                                        edges_original.iter().cloned().collect();
                                    let mut ai = peen[i].clone();
                                    let mut aj = peen[j].clone();
                                    let mut ak = peen[k].clone();
                                    let mut al = peen[l].clone();
                                    let mut am = peen[m].clone();
                                    let mut an = peen[n].clone();
                                    let mut ao = peen[o].clone();
                                    let mut ap = peen[p].clone();
                                    std::mem::swap(&mut ai.3, &mut aj.3);
                                    std::mem::swap(&mut ak.3, &mut al.3);
                                    std::mem::swap(&mut am.3, &mut an.3);
                                    std::mem::swap(&mut ao.3, &mut ap.3);
                                    peen[i] = ai;
                                    peen[j] = aj;
                                    peen[k] = ak;
                                    peen[l] = al;
                                    peen[m] = am;
                                    peen[n] = an;
                                    peen[o] = ao;
                                    peen[p] = ap;
                                    let mut edges: HashSet<(String, String, String, String)> =
                                        peen.iter().cloned().collect();

                                    loop {
                                        let to_remove: Vec<(String, String, String, String)> =
                                            edges
                                                .iter()
                                                .filter_map(|e| {
                                                    if !vals.contains_key(&e.0)
                                                        || !vals.contains_key(&e.1)
                                                    {
                                                        return None;
                                                    }
                                                    let a = vals.get(&e.0).unwrap();
                                                    let b = vals.get(&e.1).unwrap();
                                                    match e.2.chars().next().unwrap() {
                                                        'A' => vals.insert(e.3.clone(), *a & *b),
                                                        'O' => vals.insert(e.3.clone(), *a | *b),
                                                        'X' => vals.insert(e.3.clone(), *a ^ *b),
                                                        _ => panic!("E"),
                                                    };
                                                    Some(e.clone())
                                                })
                                                .collect();
                                        if to_remove.is_empty() {
                                            break;
                                        }
                                        to_remove.iter().for_each(|e| {
                                            edges.remove(e);
                                        });
                                    }
                                    let x: usize = vals
                                        .iter()
                                        .map(|v| {
                                            if !v.0.contains("x") || *v.1 == 0 {
                                                return 0;
                                            }
                                            let x = 2usize
                                                .pow(v.0.as_str()[1..3].parse::<u32>().unwrap());
                                            x
                                        })
                                        .sum();

                                    let y: usize = vals
                                        .iter()
                                        .map(|v| {
                                            if !v.0.contains("y") || *v.1 == 0 {
                                                return 0;
                                            }
                                            let x = 2usize
                                                .pow(v.0.as_str()[1..3].parse::<u32>().unwrap());
                                            x
                                        })
                                        .sum();

                                    let z = vals
                                        .iter()
                                        .map(|v| {
                                            if !v.0.contains("z") || *v.1 == 0 {
                                                return 0;
                                            }
                                            let x = 2usize
                                                .pow(v.0.as_str()[1..3].parse::<u32>().unwrap());
                                            x
                                        })
                                        .sum();
                                    if x + y == z {
                                        println!(
                                            "{}, {}, {}, {}, {}, {}, {}, {}",
                                            edges_original.iter().nth(i).unwrap().3,
                                            edges_original.iter().nth(j).unwrap().3,
                                            edges_original.iter().nth(k).unwrap().3,
                                            edges_original.iter().nth(l).unwrap().3,
                                            edges_original.iter().nth(m).unwrap().3,
                                            edges_original.iter().nth(n).unwrap().3,
                                            edges_original.iter().nth(o).unwrap().3,
                                            edges_original.iter().nth(p).unwrap().3
                                        );
                                        println!("WE FOUND IT BOYS");
                                        panic!("DONE");
                                    }
                                    //println!("{}, {}, {}", x, y, z);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    0
}
