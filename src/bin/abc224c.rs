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
    let mut pp: Vec<(isize, isize)> = vec![];
    for _ in 0..n {
        pp.push(parse_line().unwrap());
    }

    let mut ans = 0;
    for triple in pp.into_iter().combinations(3) {
        let (a, b, c) = (triple[0], triple[1], triple[2]);
        let uex = b.0 - a.0;
        let uey = b.1 - a.1;
        let sitax = b.0 - c.0;
        let sitay = b.1 - c.1;

        if sitay * uex == sitax * uey {
            continue;
        }
        ans += 1;
    }
    println!("{}", ans);
}
