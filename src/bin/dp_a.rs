use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let n: usize = parse_line().unwrap();
    let hh: Vec<isize> = parse_line().unwrap();

    let mut dp: Vec<usize> = vec![];
    dp.push(0);
    dp.push((hh[0] - hh[1]).abs() as usize);
    for i in 2..n {
        dp.push(min(
                dp[i - 1] + (hh[i] - hh[i - 1]).abs() as usize,
            dp[i - 2] + (hh[i] - hh[i - 2]).abs() as usize,
        ));
    }

    println!("{}", dp[n - 1]);
}
