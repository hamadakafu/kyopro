use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let mut aaa: Vec<Vec<isize>> = vec![];
    for _ in 0..h {
        aaa.push(parse_line().unwrap());
    }
    let mut bbb: Vec<Vec<isize>> = vec![];
    for _ in 0..h {
        bbb.push(parse_line().unwrap());
    }

    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let diff = bbb[i][j] - aaa[i][j];
            ans += diff.abs();
            for (di, dj) in vec![(0, 0), (0, 1), (1, 0), (1, 1)] {
                aaa[i + di][j + dj] += diff;
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if aaa[i][j] != bbb[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
    println!("{}", ans);
}
