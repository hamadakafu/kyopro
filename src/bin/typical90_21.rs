use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;
fn alphabet2idx(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as u8 as usize - 'a' as u8 as usize
    } else if c.is_ascii_uppercase() {
        c as u8 as usize - 'A' as u8 as usize
    } else {
        panic!("wtf")
    }
}
// SCC: 強連結成分
// https://hkawabata.github.io/technical-note/note/Algorithm/graph/scc.html
fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
    }

    // 帰り順にindexに数字が入る
    let mut already: Vec<bool> = vec![false; n + 1];
    let mut index: Vec<usize> = vec![0; n + 1];
    let mut counter = 0;
    for i in 1..=n {
        if already[i] {
            continue;
        }
        already[i] = true;
        dfs(&mut index, &mut counter, i, &mut already, &paths);
        counter += 1;
        index[counter] = i;
    }
    let mut invpaths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for (s, sp) in paths.iter().enumerate() {
        for g in sp.iter() {
            invpaths[*g].push(s);
        }
    }

    // 大きい数字からDFS(invpathsのグラフにおいて最下流からDFS)
    let mut already = vec![false; n + 1];
    let mut groups: Vec<Vec<usize>> = vec![];
    for i in (1..=n).rev() {
        let s = index[i];
        let mut group = vec![];
        if already[s] {
            continue;
        }
        let mut stack = vec![s];
        already[s] = true;
        while !stack.is_empty() {
            let t = stack.pop().unwrap();
            group.push(t);
            for p in invpaths[t].iter() {
                if already[*p] {
                    continue;
                }
                stack.push(*p);
                already[*p] = true;
            }
        }
        groups.push(group);
    }

    // 最後に何通りの(x, y)があるか?
    let mut ans = 0;
    for g in groups {
        if g.len() > 1 {
            ans += g.len() * (g.len() - 1) / 2;
        }
    }
    println!("{}", ans);
}

fn dfs(
    index: &mut Vec<usize>,
    counter: &mut usize,
    now: usize,
    already: &mut Vec<bool>,
    paths: &Vec<Vec<usize>>,
) {
    for p in paths[now].iter() {
        if !already[*p] {
            already[*p] = true;
            dfs(index, counter, *p, already, paths);
            *counter += 1;
            index[*counter] = *p;
        }
    }
}
