use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;
// dp
// https://atcoder.jp/contests/joi2013yo/tasks/joi2013yo_d

fn main() {
    let (d, n): (usize, usize) = parse_line().unwrap();
    let mut tt: Vec<usize> = vec![];
    for _ in 0..d {
        tt.push(parse_line().unwrap());
    }
    let mut fukus: Vec<(usize, usize, usize)> = vec![];
    for _ in 0..n {
        fukus.push(parse_line().unwrap());
    }

    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; n]; d + 1];
    for j in 0..n {
        for one in 0..n {
            if fukus[one].0 <= tt[0]
                && tt[0] <= fukus[one].1
                && fukus[j].0 <= tt[1]
                && tt[1] <= fukus[j].1
            {
                dp[2][j] = Some(max(
                    dp[2][j].unwrap_or(0),
                    (fukus[one].2 as isize - fukus[j].2 as isize).abs() as usize,
                ));
            }
        }
    }

    for i in 3..=d {
        for j in 0..n {
            for one in 0..n {
                if fukus[j].0 <= tt[i - 1] && tt[i - 1] <= fukus[j].1 && dp[i - 1][one].is_some() {
                    dp[i][j] = Some(max(
                        dp[i][j].unwrap_or(0),
                        dp[i - 1][one].unwrap()
                            + (fukus[one].2 as isize - fukus[j].2 as isize).abs() as usize,
                    ));
                }
            }
        }
    }
    println!("{}", dp[d].iter().map(|a| a.unwrap_or(0)).max().unwrap());
}
