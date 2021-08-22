use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let n: usize = parse_line().unwrap();
    let s: String = parse_line().unwrap();
    let s = s.chars().collect_vec();

    let mut xixi: Vec<usize> = vec![];
    let mut tmp = std::usize::MAX;
    for (i, c) in s.iter().enumerate().rev() {
        if std::usize::MAX == tmp && *c == 'o' {
            continue;
        }
        if *c == 'x' {
            tmp = i;
        }
        if *c == 'o' {
            xixi.push(tmp);
        }
    }
    let mut oioi: Vec<usize> = vec![];
    let mut tmp = std::usize::MAX;
    for (i, c) in s.iter().enumerate().rev() {
        if std::usize::MAX == tmp && *c == 'x' {
            continue;
        }
        if *c == 'o' {
            tmp = i;
        }
        if *c == 'x' {
            oioi.push(tmp);
        }
    }
    // dbg文遅いのでコメントアウトする必要がある
    // dbg!(&s, &xixi, &oioi);
    let mut ans = 0;
    for xi in xixi {
        ans += n - xi;
    }
    for oi in oioi {
        ans += n - oi;
    }

    println!("{}", ans);
}
