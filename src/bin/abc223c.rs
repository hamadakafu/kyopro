#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut abab: VecDeque<(f64, f64)> = VecDeque::new();
    for _ in 0..n {
        abab.push_back(parse_line().unwrap());
    }
    // TODO: n = 1の場合競争
    if n == 1 {
        println!("{}", abab[0].0 / 2.0);
        return;
    }
    let mut lline = abab.pop_front().unwrap();
    let mut rline = abab.pop_back().unwrap();
    let mut lbyo: f64 = lline.0 / lline.1;
    let mut rbyo: f64 = rline.0 / rline.1;

    let mut burned = 0.0;
    while !abab.is_empty() {
        if lbyo > rbyo {
            rline = abab.pop_back().unwrap();
            rbyo += rline.0 / rline.1;
        } else {
            burned += lline.0;
            lline = abab.pop_front().unwrap();
            lbyo += lline.0 / lline.1;
        }
    }
    if (lbyo - rbyo).abs() < std::f64::EPSILON {
        println!("{}", burned + lline.0);
    } else if lbyo > rbyo {
        burned += lline.0 - (lbyo - rbyo) * lline.1;
        let half = (lbyo - rbyo) * lline.1 / 2.0;
        println!("{}", burned + half);
    } else {
        burned += lline.0;
        let half = ((rbyo - lbyo) * rline.1) / 2.0;
        println!("{}", burned + half);
    }
}
