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
const mmm: usize = 998244353;

fn main() {
    let n: usize = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();
    let bb: Vec<usize> = parse_line().unwrap();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3000 + 1]; n + 1];

    if aa[0] == 0 {
        dp[1][0] = 1;
    }
    for i in 1..3001 {
        if aa[0] <= i && i <= bb[0] {
            dp[1][i] = dp[1][i - 1] + 1;
        } else {
            dp[1][i] = dp[1][i - 1];
        }
    }
    // println!("{:?}", dp[1]);
    for i in 2..=n {
        if aa[i - 1] == 0 {
            dp[i][0] = 1;
        }
        for j in 1..=*bb.last().unwrap() {
            if j < aa[i - 1] || bb[i - 1] < j {
                dp[i][j] = dp[i][j - 1];
                continue;
            }
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            dp[i][j] %= mmm;
        }
    }
    println!("{}", dp[n][*bb.last().unwrap()]);
}
