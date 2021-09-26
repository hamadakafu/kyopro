use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;

// 木を工夫して順番ずつたどることで前の値を活かしていけばO(N)で計算できる
fn main() {
    let n: usize = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for p in 0..n - 1 {
        let (u, v): (usize, usize) = parse_line().unwrap();
        paths[u].push(v);
        paths[v].push(u);
    }

    let mut already = HashSet::new();
    let mut stack = vec![(1, 0)];
    let mut ans = vec![0; n + 1];
    already.insert(1);
    // この段階で1は調査済みであるので，
    // 1に対して処理すべきことは済ましておく
    while !stack.is_empty() {
        let t = stack.pop().unwrap();
        ans[1] += t.1;
        for p in paths[t.0].iter() {
            if !already.insert(*p) {
                continue;
            }
            stack.push((*p, t.1 + 1));
            // pに対してなんかする
        }
    }

    let mut subs = vec![0; n + 1];
    let mut already = vec![false; n + 1];
    already[0] = true;
    already[1] = true;
    ki_dp(&paths, &mut already, &mut subs, 1);

    let mut already = HashSet::new();
    let mut stack = vec![1];
    already.insert(1);
    // この段階で1は調査済みであるので，
    // 1に対して処理すべきことは済ましておく
    while !stack.is_empty() {
        let t = stack.pop().unwrap();
        for p in paths[t].iter() {
            if !already.insert(*p) {
                continue;
            }
            stack.push(*p);
            ans[*p] = ans[t] + n - 2 * subs[*p];
        }
    }
    for i in 1..=n {
        println!("{}", ans[i]);
    }
}

fn ki_dp(paths: &Vec<Vec<usize>>, already: &mut Vec<bool>, memo: &mut Vec<usize>, now: usize) {
    memo[now] = 1;
    for &p in paths[now].iter() {
        if already[p] {
            continue;
        }
        already[p] = true;
        ki_dp(paths, already, memo, p);
        memo[now] += memo[p];
    }
}
