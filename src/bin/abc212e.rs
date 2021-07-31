use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;
fn alphabet2idx(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as u8 as usize - 'a' as u8 as usize
    } else if c.is_ascii_uppercase() {
        c as u8 as usize - 'A' as u8 as usize
    } else {
        panic!("wtf")
    }
}

const mmm: usize = 998244353;

fn main() {
    let (n, m, k): (usize, usize, usize) = parse_line().unwrap();
    let mut not_paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let (u, v): (usize, usize) = parse_line().unwrap();
        not_paths[u].push(v);
        not_paths[v].push(u);
    }
    for j in 1..n + 1 {
        not_paths[j].push(j);
    }

    let mut memo: Vec<Vec<usize>> = vec![vec![0; n + 1]; k + 1];
    for j in 1..n + 1 {
        memo[1][j] = 1;
        if not_paths[1].contains(&j) {
            memo[1][j] -= 1;
        }
    }

    for i in 2..k + 1 {
        let presum: usize = memo[i - 1].iter().sum();
        for j in 1..n + 1 {
            let mut tmp = 0;
            for np in not_paths[j].iter() {
                tmp += memo[i - 1][*np];
            }
            memo[i][j] = presum - tmp;
            memo[i][j] %= mmm;
        }
    }
    println!("{}", memo[k][1]);
}
