use std::collections::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn dfs(
    start: String,
    curr: String,
    edges: &HashMap<String, Vec<String>>,
    paths: &mut HashSet<(String, String, String)>,
    mut curr_path: Vec<String>,
) {
    if curr_path.len() == 3 {
        if curr == start {
            curr_path.sort();
            paths.insert((
                curr_path[0].clone(),
                curr_path[1].clone(),
                curr_path[2].clone(),
            ));
        }
        return;
    }
    edges.get(&curr).unwrap().iter().for_each(|e| {
        curr_path.push(e.clone());
        dfs(start.clone(), e.clone(), edges, paths, curr_path.clone());
        curr_path.pop();
    });
}

pub fn part_1(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    reader.lines().map(|l| l.unwrap()).for_each(|l| {
        let mut b = l.split_terminator("-");
        let f = b.next().unwrap().to_owned();
        let g = b.next().unwrap().to_owned();
        edges.entry(f.clone()).or_default().push(g.clone());
        edges.entry(g).or_default().push(f);
    });
    let mut paths: HashSet<(String, String, String)> = HashSet::new();
    edges.iter().for_each(|(k, v)| {
        if !k.starts_with("t") {
            return;
        }
        v.iter().for_each(|e| {
            dfs(k.clone(), e.clone(), &edges, &mut paths, vec![e.clone()]);
        });
    });
    paths.len()
}

fn bron_kerbosch(
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    edges: &HashMap<String, HashSet<String>>,
    best_path: &mut Vec<String>,
) {
    if p.is_empty() && x.is_empty() {
        if best_path.len() < r.len() {
            *best_path = Vec::from_iter(r.iter().cloned());
        }
        return;
    }

    let pivot = p.union(x).next().unwrap();
    let l: Vec<String> = p.difference(edges.get(pivot).unwrap()).cloned().collect();
    for v in &l {
        let mut new_r = r.clone();
        new_r.insert(v.clone());
        let mut new_p: HashSet<String> = p.intersection(edges.get(v).unwrap()).cloned().collect();
        let mut new_x: HashSet<String> = x.intersection(edges.get(v).unwrap()).cloned().collect();

        bron_kerbosch(&mut new_r, &mut new_p, &mut new_x, edges, best_path);

        p.remove(v);
        x.insert(v.clone());
    }
}

pub fn part_2(file: String) -> usize {
    let reader = BufReader::new(File::open(file).unwrap());
    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
    let mut p: HashSet<String> = HashSet::new();
    reader.lines().map(|l| l.unwrap()).for_each(|l| {
        let mut b = l.split_terminator("-");
        let f = b.next().unwrap().to_owned();
        let g = b.next().unwrap().to_owned();
        edges.entry(f.clone()).or_default().insert(g.clone());
        edges.entry(g.clone()).or_default().insert(f.clone());
        p.insert(f.clone());
        p.insert(g.clone());
    });
    let mut best_path: Vec<String> = vec![];
    bron_kerbosch(
        &mut HashSet::new(),
        &mut p,
        &mut HashSet::new(),
        &edges,
        &mut best_path,
    );

    best_path.sort();
    best_path.iter().for_each(|i| {
        print!("{i},");
    });
    println!();
    best_path.len()
}
