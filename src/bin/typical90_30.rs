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
    let tmp = eratosthenes(n);
    let mut ans = 0;
    for t in tmp.iter().skip(1) {
        if *t >= k {
            ans += 1;
        }
    }
    println!("{}", ans);
}

/// O(NloglogN)
/// 1からnまでのすべての数に対してそれぞれ約数の個数を数える
fn eratosthenes(n: usize) -> Vec<usize> {
    let mut ans = vec![0; n];
    for i in 2..=n {
        if ans[i - 1] != 0 {
            continue;
        }
        let mut j = i;
        while j <= n {
            ans[j - 1] += 1;
            j += i;
        }
    }
    ans
}
