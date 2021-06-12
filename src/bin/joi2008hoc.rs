use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut pp: Vec<usize> = vec![];
    for _ in 0..n {
        pp.push(parse_line().unwrap());
    }

    let mut two = vec![];
    for i in 0..n + 1 {
        for j in 0..n + 1 {
            let iv = if i == n { 0 } else { pp[i] };
            let jv = if j == n { 0 } else { pp[j] };
            two.push(iv + jv);
        }
    }
    two.sort_by(|a, b| b.cmp(&a));

    let mut ans = 0;
    for j in two.iter() {
        if j > &m {
            continue;
        }
        let mut left = 0;
        let mut right = two.len();
        while left < right {
            let pivot = (right + left) / 2;
            if two[pivot] + j <= m {
                right = pivot;
            } else {
                left = pivot + 1;
            }
        }
        if left >= two.len() {
            left = two.len() - 1;
        }
        ans = max(ans, two[left] + j);
    }
    println!("{}", ans);
}

