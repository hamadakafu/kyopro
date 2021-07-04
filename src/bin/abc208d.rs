use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

// ワーシャル フロイド 法
// kを中継地点としてkを小さい順にO(n^3)で最短路を求めるdpを使っている方法
// 本当はn * nのdpで十分
fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut map: Vec<Vec<usize>> = vec![vec![std::usize::MAX; n + 1]; n + 1];
    for _ in 0..m {
        let (a, b, c): (usize, usize, usize) = parse_line().unwrap();
        map[a][b] = c;
    }

    let mut memo: Vec<Vec<Vec<usize>>> = vec![vec![vec![std::usize::MAX; n + 1]; n + 1]; n + 1];

    for s in 1..=n {
        for t in 1..=n {
            memo[s][t][0] = map[s][t];
        }
    }
    for k in 0..=n {
        for s in 1..=n {
            memo[s][s][k] = 0;
        }
    }
    let mut ans = 0;
    for k in 1..=n {
        for s in 1..=n {
            for t in 1..=n {
                if s == t {
                    continue;
                }
                if memo[s][k][k - 1] == std::usize::MAX || memo[k][t][k - 1] == std::usize::MAX {
                    memo[s][t][k] = memo[s][t][k - 1];
                } else {
                    memo[s][t][k] = min(memo[s][k][k - 1] + memo[k][t][k - 1], memo[s][t][k - 1]);
                }
                if memo[s][t][k] == std::usize::MAX {
                    continue;
                }
                ans += memo[s][t][k];
            }
        }
    }
    println!("{}", ans);
}

