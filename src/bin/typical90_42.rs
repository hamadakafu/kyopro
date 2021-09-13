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
    let k: usize = parse_line().unwrap();
    // dp[i]は合計がiになる数の個数
    let mut dp: Vec<usize> = vec![1];
    let mut i = 1;
    while i <= k {
        let mut tmp = 0;
        for diff in 1..=9 {
            if i >= diff {
                tmp += dp[i - diff];
                tmp %= ten97;
            }
        }
        dp.push(tmp);
        i += 1;
    }
    if k % 9 == 0 {
        println!("{}", dp[k]);
    } else {
        println!("0");
    }
}
