use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..m {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
        paths[b].push(a);
    }

    let mut ans = 0;
    for (i, path) in paths.into_iter().enumerate() {
        if path.into_iter().filter(|a| *a < i).count() == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
