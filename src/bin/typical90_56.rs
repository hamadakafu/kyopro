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
    let (n, s): (usize, usize) = parse_line().unwrap();
    let mut abab: Vec<(usize, usize)> = vec![];
    for _ in 0..n {
        abab.push(parse_line().unwrap());
    }

    let mut dp: Vec<Vec<char>> = vec![vec!['a'; s + 1]; n + 1];
    // 1日目の初期化
    if abab[0].0 <= s {
        dp[1][abab[0].0] = 'A';
    }
    if abab[0].1 <= s {
        dp[1][abab[0].1] = 'B';
    }
    for day in 2..=n {
        for en in 1..=s {
            if en + abab[day - 1].0 <= s && (dp[day - 1][en] == 'A' || dp[day - 1][en] == 'B') {
                dp[day][en + abab[day - 1].0] = 'A';
            }
            if en + abab[day - 1].1 <= s && (dp[day - 1][en] == 'A' || dp[day - 1][en] == 'B') {
                dp[day][en + abab[day - 1].1] = 'B';
            }
        }
    }

    if dp[n][s] == 'a' {
        println!("Impossible");
        return;
    }
    let mut now = s;
    let mut ans = "".to_owned();
    for day in (1..=n).rev() {
        if dp[day][now] == 'A' {
            now -= abab[day - 1].0;
            ans.push('A');
        } else if dp[day][now] == 'B' {
            now -= abab[day - 1].1;
            ans.push('B');
        } else {
            panic!("wtf");
        }
    }
    println!("{}", ans.chars().rev().collect::<String>());
}
