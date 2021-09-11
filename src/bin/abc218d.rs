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
    let mut points = vec![];
    for _ in 0..n {
        let tmp: (usize, usize) = parse_line().unwrap();
        points.push(tmp);
    }
    points.sort();

    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            let hidariue = points[i];
            let migishita = points[j];
            if hidariue.0 >= migishita.0 {
                continue;
            }
            if hidariue.1 <= migishita.1 {
                continue;
            }
            let migiue = (migishita.0, hidariue.1);
            let hidarishita = (hidariue.0, migishita.1);
            if points.binary_search(&migiue).is_ok() && points.binary_search(&hidarishita).is_ok() {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
