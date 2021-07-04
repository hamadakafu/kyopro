use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

// dp
// https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_d
// j番目の列をある色で塗ったときにかかるコスト

fn main() {
    let n: usize = parse_line().unwrap();
    let mut hatas: Vec<Vec<char>> = vec![];
    for _ in 0..5 {
        let tmp: String = parse_line().unwrap();
        hatas.push(tmp.chars().collect_vec());
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n];
    for c in 0..3 {
        dp[0][c] = change(color(c), 0, &hatas);
    }
    for j in 1..n {
        for c in 0..3 {
            let mut tmp = std::usize::MAX;
            for maec in 0..3 {
                if c != maec {
                    tmp = min(tmp, dp[j - 1][maec] + change(color(c), j, &hatas));
                }
            }
            dp[j][c] = tmp;
        }
    }
    println!("{}", dp[n-1].iter().min().unwrap());
}

fn color(c: usize) -> char {
    match c {
        0 => 'R',
        1 => 'B',
        2 => 'W',
        _ => panic!("wtf"),
    }
}

// 色に応じてj列(0始まり)が何個変える必要があるか
fn change(c: char, j: usize, hatas: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    for i in 0..5 {
        if hatas[i][j] != c {
            ans += 1;
        }
    }
    ans
}
