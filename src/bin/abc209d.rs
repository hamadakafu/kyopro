use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use whiteread::parse_line;

// 木は2部グラフであるので赤と青にノードを色分けすればTownかRoadかわかる

fn main() {
    let (n, q): (usize, usize) = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
        paths[b].push(a);
    }

    let mut iro: Vec<usize> = vec![0; n + 1];
    dfs(&paths, &mut iro);
    for _ in 0..q {
        let (c, d): (usize, usize) = parse_line().unwrap();
        if iro[c] == iro[d] {
            println!("Town");
        } else {
            println!("Road");
        }
    }
    dbg!(&iro);
}

fn dfs(paths: &Vec<Vec<usize>>, iro: &mut Vec<usize>) {
    let mut already = HashSet::new();
    let mut stack = vec![1];
    already.insert(1);
    while !stack.is_empty() {
        let t = stack.pop().unwrap();
        for p in paths[t].iter() {
            if !already.insert(*p) {
                continue;
            }
            stack.push(*p);
            iro[*p] = 1 - iro[t];
        }
    }
}
