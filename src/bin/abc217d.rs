#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::borrow::Borrow;
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (l, q): (isize, isize) = parse_line().unwrap();
    let mut bts = BTreeSet::new();
    bts.insert(0);
    bts.insert(l);
    for _ in 0..q {
        let (c, x): (usize, isize) = parse_line().unwrap();
        if c == 1 {
            bts.insert(x);
        } else {
            let l = bts.range(..x).next_back().unwrap();
            let r = bts.range(x..).next().unwrap();
            println!("{}", r - l);
        }
    }
}
