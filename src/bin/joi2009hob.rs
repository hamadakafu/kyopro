use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let ensyu: i64 = parse_line().unwrap();
    let n: usize = parse_line().unwrap();
    let m: usize = parse_line().unwrap();
    let mut tenpo: Vec<(usize, i64)> = vec![(1, 0), (1, ensyu)];
    for i in 0..n - 1 {
        let dis: i64 = parse_line().unwrap();
        tenpo.push((i + 2, dis));
    }

    tenpo.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ans = 0;
    for _ in 0..m {
        let dis: i64 = parse_line().unwrap();
        let uber = find_zero(&tenpo, dis);
        ans += (dis - uber.1).abs();

    }
    println!("{}", ans);
}

fn find_zero(tenpo: &Vec<(usize, i64)>, value: i64) -> (usize, i64) {
    let mut left = 0;
    let mut right = tenpo.len();
    while left != right {
        let pivot = (right + left) / 2;
        if value <= tenpo[pivot].1 {
            right = pivot;
        } else {
            left = pivot + 1;
        }
    }

    if left == tenpo.len() {
        return tenpo[left - 1];
    }

    if left > 0 {
        if (tenpo[left].1 - value).abs() > (tenpo[left - 1].1 - value).abs() {
            return tenpo[left - 1];
        }
    }
    return tenpo[left];
}
