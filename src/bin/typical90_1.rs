use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

// 出力で2部探索
// pivotでcheckする関数が簡単に計算できるのが重要かも知れない

fn main() {
    let (n, l): (usize, usize) = parse_line().unwrap();
    let k: usize = parse_line().unwrap();
    let mut aa: Vec<usize> = parse_line().unwrap();
    aa.push(l);

    // upper bounds
    let mut left = 1;
    let mut right = l;
    while left < right {
        let pivot = (right + left) / 2;
        if !check(pivot, &aa, k, l) {
            right = pivot;
        } else {
            left = pivot + 1;
        }
    }
    println!("{}", left - 1);
}

fn check(pivot: usize, aa: &Vec<usize>, k: usize, l: usize) -> bool {
    let mut cnt: usize = 0;
    let mut aaindex: usize = 0;
    let mut now = 0;
    while cnt < k + 1 {
        if aaindex == aa.len() {
            return false;
        }
        if aa[aaindex] - now >= pivot {
            now = aa[aaindex];
            cnt += 1;
        }
        aaindex += 1;
    }
    true
}

