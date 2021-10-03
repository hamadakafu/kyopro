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
    let n: String = parse_line().unwrap();
    let n: Vec<u32> = n
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sorted_by(|a, b| b.cmp(&a))
        .collect_vec();
    let vec = &n;
    let mut ans = 0;
    for mut bits in 0..2_u64.pow(vec.len() as u32) {
        let mut vv = vec![];
        let mut nonvv = vec![];
        for i in 0..vec.len() {
            if bits % 2 == 1 {
                vv.push(vec[i]);
            } else {
                nonvv.push(vec[i]);
            }
            bits /= 2;
        }
        // nonvv.sort_by(|a, b| b.cmp(&a));
        // dbg!(&vv, &nonvv, vectonum(&vv));
        let left = vectonum(&vv);
        let right = vectonum(&nonvv);
        if left == 0 || right == 0 {
            continue;
        }
        ans = ans.max(left * right);
    }
    println!("{}", ans);
}

fn vectonum(v: &Vec<u32>) -> usize {
    let mut ans = 0;
    for vv in v.iter() {
        ans *= 10;
        ans += *vv as usize;
    }
    ans
}
