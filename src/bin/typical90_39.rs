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

fn main() {
    let n: usize = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
        paths[b].push(a);
    }

    let mut memo = vec![0; n + 1];
    let mut already = vec![false; n + 1];
    already[0] = true;
    already[1] = true;
    ki_dp(&paths, &mut already, &mut memo, 1);
    let mut ans = 0;
    for m in memo.into_iter().skip(1) {
        ans += m * (n - m);
    }
    println!("{}", ans);
}

/// 各ノードが持つ部下の数のを求める(自分含む)
/// indexは1始まり
/// dp[x] = 1 + dp[y_1] + dp[y_2] + dp[y_3] ...
/// ```
/// let mut memo = vec![0; n + 1];
/// let mut already = vec![false; n + 1];
/// already[0] = true;
/// already[1] = true;
/// ki_dp(&paths, &mut already, &mut memo, 1);
/// ```
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
