use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut hshs: Vec<(u64, u64)> = vec![];
    for _ in 0..n {
        let hs = parse_line().unwrap();
        hshs.push(hs);
    }
    let mut maxhs = 0;
    for (h, s) in hshs.iter() {
        maxhs = max(h + (n as u64 - 1) * s, maxhs);
    }

    let mut left = 0;
    let mut right = maxhs;
    while left < right {
        let pivot = (right + left) / 2;
        if check(pivot, &hshs) {
            right = pivot;
        } else {
            left = pivot + 1;
        }
    }
    println!("{}", left);
}

fn check(target: u64, hshs: &[(u64, u64)]) -> bool {
    let mut aa = vec![];
    for &(h, s) in hshs.iter() {
        if h > target {
            return false;
        } else {
            aa.push((target - h) / s);
        }
    }
    aa.sort();
    for (a, ni) in aa.iter().zip(0..hshs.len()) {
        if *a < ni as u64 {
            return false;
        }
    }
    true
}

