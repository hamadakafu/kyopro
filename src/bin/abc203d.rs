// 2重累積和と2部探索
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let mut aaa: Vec<Vec<u64>> = vec![];

    for _ in 0..n {
        let line = parse_line().unwrap();
        aaa.push(line);
    }

    let tyuo = (k * k) as u64 / 2 + 1;

    let mut left = 0;
    let mut right = *aaa.iter().map(|v| v.iter().max().unwrap()).max().unwrap();

    while left < right {
        let pivot = (right + left) / 2;
        let mut ss: Vec<Vec<u64>> = vec![vec![]; n + 1];
        for i in 0..n + 1 {
            ss[i].push(0);
        }
        ss[0] = vec![0; n + 1];

        for i in 1..n + 1 {
            for j in 1..n + 1 {
                if aaa[i - 1][j - 1] > pivot {
                    let tmp = ss[i - 1][j] + ss[i][j - 1] - ss[i - 1][j - 1] + 1;
                    ss[i].push(tmp);
                } else {
                    let tmp = ss[i - 1][j] + ss[i][j - 1] - ss[i - 1][j - 1];
                    ss[i].push(tmp);
                }
            }
        }

        let mut minrange = std::u64::MAX;
        for i in k..n + 1 {
            for j in k..n + 1 {
                let range_num = ss[i][j] - ss[i - k][j] - ss[i][j - k] + ss[i - k][j - k];
                minrange = min(range_num, minrange);
            }
        }
        if minrange < tyuo {
            right = pivot;
        } else {
            left = pivot + 1;
        }
    }
    println!("{}", left);
}
