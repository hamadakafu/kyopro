use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let (s, k): (String, usize) = parse_line().unwrap();
    let mut s: Vec<char> = s.chars().collect_vec();
    s.sort();
    let tmp = dfs(s);
    let ans: String = tmp[k - 1].iter().collect();
    println!("{}", ans);
}

fn dfs(s: Vec<char>) -> Vec<VecDeque<char>> {
    let mut tmp = vec![];
    if s.len() == 1 {
        return vec![VecDeque::from(s)];
    }
    let mut already: HashSet<char> = HashSet::new();
    for (i, c) in s.iter().enumerate() {
        if !already.insert(*c) {
            continue;
        }
        let mut t = s.clone();
        t.remove(i);
        let a = dfs(t)
            .into_iter()
            .map(|mut v| {
                v.push_front(*c);
                v
            })
            .collect_vec();
        tmp.extend(a);
    }
    tmp
}
