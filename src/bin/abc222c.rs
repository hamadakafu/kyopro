#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

use itertools::{sorted, Itertools};
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut aaa: Vec<Vec<char>> = vec![];
    for _ in 0..2 * n {
        let aa: String = parse_line().unwrap();
        let aa: Vec<char> = aa.chars().collect_vec();
        aaa.push(aa);
    }
    let mut katikazu: Vec<(Reverse<usize>, usize)> = vec![];
    for i in 1..=2 * n {
        katikazu.push((Reverse(0), i));
    }
    for j in 0..m {
        katikazu.sort();
        for i in 0..n {
            let a = katikazu[2 * i];
            let b = katikazu[2 * i + 1];
            let (aa, bb) = battle(aaa[a.1 - 1][j], aaa[b.1 - 1][j]);
            let tmp = katikazu[2 * i].0;
            katikazu[2 * i].0 = Reverse(tmp.0 + aa);
            let tmp = katikazu[2 * i + 1].0;
            katikazu[2 * i + 1].0 = Reverse(tmp.0 + bb);
        }
        // println!("{:?}", &katikazu);
    }
    katikazu.sort();
    for k in 0..2 * n {
        println!("{}", katikazu[k].1);
    }
}

fn battle(a: char, b: char) -> (usize, usize) {
    if (a == 'G' && b == 'C') || (a == 'C' && b == 'P') || (a == 'P' && b == 'G') {
        (1, 0)
    } else if (b == 'G' && a == 'C') || (b == 'C' && a == 'P') || (b == 'P' && a == 'G') {
        (0, 1)
    } else {
        (0, 0)
    }
}
