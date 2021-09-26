#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::fmt::Debug;
use std::cmp::Reverse;
use std::cmp::{min, max};
use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let n: usize = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();
    let x: usize = parse_line().unwrap();
    let sumaa: usize = aa.iter().sum();

    let mut tmp = x / sumaa;
    let mut ans = tmp * n;
    let mut xx = x - tmp * sumaa;


    for a in aa.into_iter() {
        ans += 1;
        if xx < a {
            break;
        }
        xx -= a;
    }
    println!("{}", ans);
}

