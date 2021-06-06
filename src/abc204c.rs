use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
    }
    let mut ans = 0;
    for i in 1..=n {
        let mut h = HashSet::new();
        ans += dfs(0, i, &paths, &mut h);
    }
    println!("{}", ans);
}

fn dfs(
    mut current: usize,
    target: usize,
    paths: &Vec<Vec<usize>>,
    checked: &mut HashSet<usize>,
) -> usize {
    if checked.contains(&target) {
        return current;
    }
    checked.insert(target);
    current += 1;

    for p in paths[target].iter() {
        current = dfs(current, *p, paths, checked);
    }

    return current;
}
