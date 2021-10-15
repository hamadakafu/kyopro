#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

// bit毎に処理して数え上げるときにはprodするだけで場合の数になる

fn main() {
    let (n, q): (usize, usize) = parse_line().unwrap();
    let mut stst: Vec<Vec<(usize, usize, usize, bool)>> = vec![vec![]; 60];

    for _ in 0..q {
        let (x, y, z, w): (usize, usize, usize, usize) = parse_line().unwrap();
        for (i, c) in format!("{:060b}", w).char_indices() {
            let wb = if c.to_digit(2).unwrap() == 0 {
                false
            } else {
                true
            };
            stst[i].push((x - 1, y - 1, z - 1, wb));
        }
    }
    let mut ans = 1;
    for i in 0..60 {
        let mut tmp = 0;

        for mut bits in 0..2_u64.pow(n as u32) {
            let mut vv = vec![];
            for ii in 0..n {
                if bits % 2 == 1 {
                    vv.push(false);
                } else {
                    vv.push(true)
                }
                bits /= 2;
            }

            // check
            if stst[i]
                .iter()
                .all(|(x, y, z, wb)| (vv[*x] || vv[*y] || vv[*z]) == *wb)
            {
                tmp += 1;
            }
        }
        tmp %= ten97;
        ans *= tmp;
        ans %= ten97;
    }
    println!("{}", ans);
}
