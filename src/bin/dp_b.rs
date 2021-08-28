use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let hh_tmp: Vec<isize> = parse_line().unwrap();
    let mut hh = vec![0];
    hh.extend(hh_tmp);

    let mut dp: Vec<usize> = vec![std::usize::MAX; n + 1];
    dp[1] = 0;
    for i in 1..=n {
        for j in 1..=k {
            if i <= j {
                continue;
            }
            dp[i] = min(dp[i], dp[i - j] + (hh[i] - hh[i - j]).abs() as usize);
        }
    }

    println!("{}", dp[n]);
}
