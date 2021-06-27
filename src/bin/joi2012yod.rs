use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;
// dp
// https://atcoder.jp/contests/joi2012yo/tasks/joi2012yo_d

fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let mut already: HashMap<usize, usize> = HashMap::new();
    for _ in 0..k {
        let (key, v) = parse_line().unwrap();
        already.insert(key, v);
    }
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 4]; 4]; n + 1];

    if let Some(v1) = already.get(&1) {
        if let Some(v2) = already.get(&2) {
            dp[2][*v2][*v1] = 1;
        } else {
            for v2 in 1..=3 {
                dp[2][v2][*v1] = 1;
            }
        }
    } else if let Some(v2) = already.get(&2) {
        for v1 in 1..=3 {
            dp[2][*v2][v1] = 1;
        }
    } else {
        for v1 in 1..=3 {
            for v2 in 1..=3 {
                dp[2][v2][v1] = 1;
            }
        }
    }

    for i in 3..=n {
        for one in 1..=3 {
            for two in 1..=3 {
                if let Some(&yoyaku) = already.get(&i) {
                    if one == yoyaku && two == yoyaku {
                        // donothing
                    } else {
                        dp[i][yoyaku][two] += dp[i - 1][two][one];
                        dp[i][yoyaku][two] %= 10000;
                    }
                } else {
                    for today in 1..=3 {
                        if one == today && two == today {
                            // donothing
                        } else {
                            dp[i][today][two] += dp[i - 1][two][one];
                            dp[i][today][two] %= 10000;
                        }
                    }
                }
            }
        }
    }
    let ans = dp[n]
        .iter()
        .map(|a| a.iter().sum::<usize>() % 10000)
        .sum::<usize>()
        % 10000;
    println!("{}", ans);
}
