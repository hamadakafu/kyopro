use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

// FIXME: すべてのテストケースに通らない
// dfs 深さ優先探索
fn main() {
    let (n, q): (usize, usize) = parse_line().unwrap();
    // 0が余分にある
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut ctree: Vec<u64> = vec![0; n + 1];
    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = parse_line().unwrap();
        tree[a].push(b);
    }

    for _ in 0..q {
        let (p, x): (usize, u64) = parse_line().unwrap();
        ctree[p] += x;
    }

    let mut queue = vec![1];
    while !queue.is_empty() {
        let target = queue.pop().unwrap();
        queue.extend(tree[target].clone());
        for c in tree[target].iter() {
            ctree[*c] += ctree[target];
        }
    }

    for c in ctree.iter().skip(1).take(n - 1) {
        print!("{} ", c);
    }
    println!("{}", &ctree.last().unwrap());
}
