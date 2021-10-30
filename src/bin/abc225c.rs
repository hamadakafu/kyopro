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
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut bbb: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        let bb = parse_line().unwrap();
        bbb.push(bb);
    }

    let si = (bbb[0][0] - 1) / 7;
    let sj = (bbb[0][0] - 1) % 7;
    dbg!(si, sj);
    for i in si..n + si {
        for j in sj..m + sj {
            if j >= 7 {
                println!("No");
                return;
            }
            if bbb[i - si][j - sj] == (i * 7) + j + 1 {
                continue;
            } else {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
