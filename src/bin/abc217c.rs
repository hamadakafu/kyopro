use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let n: usize = parse_line().unwrap();
    let pp: Vec<usize> = parse_line().unwrap();

    let mut qq = vec![0; n];

    for (i, p) in pp.into_iter().enumerate() {
        qq[p-1] = i + 1;
    }

    print!("{}", qq[0]);
    for q in qq.into_iter().skip(1) {
        print!(" {}", q)
    }
    println!();
}

