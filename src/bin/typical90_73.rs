#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let n: usize = parse_line().unwrap();
    let cc: Vec<String> = parse_line().unwrap();
    let cc: Vec<char> = cc.into_iter().map(|a| a.chars().collect_vec()[0]).collect();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
        paths[b].push(a);
    }
    let mut memo = vec![vec![0; 3]; n + 1];
    let mut already = vec![false; n + 1];
    already[1] = true;
    ki_dp(&cc, &paths, &mut already, &mut memo, 1);
    println!("{}", memo[1][2]);
    // dbg!(memo);
}

/// 各ノードが持つ部下の数のを求める(自分含む)
/// indexは1始まり
/// dp[x] = 1 + dp[y_1] + dp[y_2] + dp[y_3] ...
/// Error: The argument 'file' cannot be empty. Received ''Error: Command failed:
/// let mut memo = vec![0; n + 1];
/// let mut already = vec![false; n + 1];
/// already[0] = true;
/// already[1] = true;
/// ki_dp(&paths, &mut already, &mut memo, 1);
fn ki_dp(
    cc: &Vec<char>,
    paths: &Vec<Vec<usize>>,
    already: &mut Vec<bool>,
    memo: &mut Vec<Vec<usize>>,
    now: usize,
) {
    // leaf
    if paths[now].iter().all(|p| already[*p]) {
        if cc[now - 1] == 'a' {
            memo[now][0] = 1;
        } else {
            memo[now][1] = 1;
        }
        return;
    }
    let mut child = vec![];
    for &p in paths[now].iter() {
        if already[p] {
            continue;
        }
        child.push(p);
        already[p] = true;
        ki_dp(cc, paths, already, memo, p);
    }
    if cc[now - 1] == 'a' {
        let mut tmp = 1;
        for mut i in child.iter().map(|&p| memo[p][0] + memo[p][2]) {
            i %= ten97;
            tmp *= i;
            tmp %= ten97;
        }
        memo[now][0] = tmp;
        let mut tmp = 1;
        for mut i in child
            .iter()
            .map(|&p| memo[p][0] + memo[p][1] + 2 * memo[p][2])
        {
            i %= ten97;
            tmp *= i;
            tmp %= ten97;
        }
        memo[now][2] = tmp + inv(memo[now][0]);
        memo[now][2] %= ten97;
    } else {
        let mut tmp = 1;
        for mut i in child.iter().map(|&p| memo[p][1] + memo[p][2]) {
            i %= ten97;
            tmp *= i;
            tmp %= ten97;
        }
        memo[now][1] = tmp;
        let mut tmp = 1;
        for mut i in child
            .iter()
            .map(|&p| memo[p][0] + memo[p][1] + 2 * memo[p][2])
        {
            i %= ten97;
            tmp *= i;
            tmp %= ten97;
        }
        memo[now][2] = tmp + inv(memo[now][1]);
        memo[now][2] %= ten97;
    }
}

fn inv(a: usize) -> usize {
    let tmp = a / ten97;
    ((tmp + 1) * ten97 - a) % ten97
}
