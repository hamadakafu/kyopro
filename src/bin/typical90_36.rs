#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (n, q): (usize, usize) = parse_line().unwrap();
    let mut xx: Vec<isize> = vec![];
    let mut yy: Vec<isize> = vec![];
    for _ in 0..n {
        let (x, y): (isize, isize) = parse_line().unwrap();

        xx.push(x - y);
        yy.push(x + y);
    }
    let xmin = *xx.iter().min().unwrap() as isize;
    let xmax = *xx.iter().max().unwrap() as isize;
    let ymin = *yy.iter().min().unwrap() as isize;
    let ymax = *yy.iter().max().unwrap() as isize;

    for _ in 0..q {
        let q: usize = parse_line().unwrap();
        println!(
            "{}",
            vec![
                (xx[q - 1] - xmin).abs(),
                (xx[q - 1] - xmax).abs(),
                (yy[q - 1] - ymin).abs(),
                (yy[q - 1] - ymax).abs()
            ]
            .iter()
            .max()
            .unwrap()
        );
    }
}
