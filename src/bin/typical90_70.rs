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
    let mut xx: Vec<isize> = vec![];
    let mut yy: Vec<isize> = vec![];
    for _ in 0..n {
        let (x, y): (isize, isize) = parse_line().unwrap();
        xx.push(x);
        yy.push(y);
    }
    xx.sort();
    yy.sort();
    let halfn = (n as f64 / 2.0).ceil() as usize;
    let nakax = xx[halfn - 1];
    let nakay = yy[halfn - 1];

    let mut ans = 0;
    for i in 0..n {
        ans += (xx[i] - nakax).abs();
        ans += (yy[i] - nakay).abs();
    }
    println!("{}", ans);
}
