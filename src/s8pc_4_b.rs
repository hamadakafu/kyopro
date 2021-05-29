use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let aa: Vec<u64> = parse_line().unwrap();

    let colors = (0..n).collect_vec();

    let mut ans = std::u64::MAX;
    for mut bits in 0..2_u64.pow(colors.len() as u32) {
        let mut vv = vec![];
        for i in 0..colors.len() {
            if bits % 2 == 1 {
                vv.push(true);
            } else {
                vv.push(false);
            }
            bits /= 2;
        }
        ans = min(ans, money(&aa, &vv, k));
    }
    println!("{}", ans);
}

fn money(aa: &Vec<u64>, colors: &Vec<bool>, k: usize) -> u64 {
    let mut aa = aa.clone();
    // 1 <= kなので存在保証
    let mut pre: usize = 0;
    let mut m = 0;
    for (i, c) in colors.iter().enumerate().skip(1) {
        if *c {
            if aa[pre] >= aa[i] {
                m += aa[pre] - aa[i] + 1;
                aa[i] = aa[pre] + 1;
            }
        }
        pre = if aa[pre] > aa[i] { pre } else { i };
    }
    // check
    let mut pre = 0;
    let mut count = 1;
    for i in 1..aa.len() {
        if aa[i] > aa[pre] {
            count += 1;
            pre = i;
        }
    }
    if count != k {
        return std::u64::MAX;
    }
    return m;
}
