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
    let mut lrlr: Vec<(usize, usize)> = vec![];
    for _ in 0..n {
        let lr: (usize, usize) = parse_line().unwrap();
        lrlr.push(lr);
    }

    let mut ans = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            let ilr = lrlr[i];
            let jlr = lrlr[j];
            let all = (1 + ilr.1 - ilr.0) * (1 + jlr.1 - jlr.0);
            let mut tmp = 0;
            for ii in ilr.0.max(jlr.0 + 1)..=ilr.1 {
                tmp += 1 + (ii - 1).min(jlr.1) - jlr.0;
            }
            ans += tmp as f64 / all as f64;
        }
    }
    println!("{}", ans);
}
