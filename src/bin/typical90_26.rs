use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

// 木が2部グラフであることを利用する
// その際綺麗に1対1に分割されるとは限らないことに注意する

fn main() {
    let n: usize = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
        paths[b].push(a);
    }

    let mut already = HashSet::new();
    let mut stack = vec![(1, true)];
    already.insert(1);
    let mut evens = vec![];
    let mut odds = vec![];
    // この段階で1は調査済みであるので，
    // 1に対して処理すべきことは済ましておく
    while !stack.is_empty() {
        let (t, b) = stack.pop().unwrap();
        if b {
            evens.push(t);
        } else {
            odds.push(t);
        }
        for p in paths[t].iter() {
            if !already.insert(*p) {
                continue;
            }
            stack.push((*p, !b));
            // pに対してなんかする
        }
    }
    dbg!(&evens, &odds);

    let ans = if evens.len() > odds.len() {
        evens
    } else {
        odds
    };
    print!("{}", ans[0]);
    for i in ans.into_iter().skip(1).take(n / 2 - 1) {
        print!(" {}", i);
    }
    println!();
}
