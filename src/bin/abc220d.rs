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

    // precal[i][j]: iが前に来ているときにxが出せる場合の数
    let mut precal: Vec<Vec<(usize, usize)>> = vec![vec![]; 10];
    for i in 0..=9 {
        for j in 0..=9 {
            let t1 = i * j % 10;
            let t2 = (i + j) % 10;
            precal[t1].push((i, j));
            precal[t2].push((i, j));
        }
    }
    //     for i in precal.iter() {
    //         println!("{:?}", &i);
    //     }

    let mut memo = vec![vec![vec![0; 10]; 10]; n];

    memo[0][aa[0]][aa[1]] = 1;
    for i in (1..n - 1) {
        for x in 0..=9 {
            for p in precal[x].iter() {
                memo[i][x][aa[i + 1]] += memo[i - 1][p.0][p.1];
                memo[i][x][aa[i + 1]] %= mmm;
            }
        }
    }
    for i in 0..=9 {
        let mut tmp = 0;
        for p in precal[i].iter() {
            tmp += memo[n - 2][p.0][p.1];
            tmp %= mmm;
        }
        println!("{}", tmp);
    }
}
