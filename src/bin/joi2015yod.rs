use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;
// dp
// https://atcoder.jp/contests/joi2015yo/tasks/joi2015yo_d
// i日目にちょうどj番目の都市につくときの疲労度
fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut dd: Vec<usize> = vec![];
    dd.push(0);
    for _ in 0..n {
        dd.push(parse_line().unwrap());
    }
    let mut cc: Vec<usize> = vec![];
    cc.push(0);
    for _ in 0..m {
        cc.push(parse_line().unwrap());
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; m + 1]; n + 1];
    dp[1][1] = dd[1] * cc[1];
    for i in 2..=m {
        dp[1][i] = min(dp[1][i - 1], dd[1] * cc[i]);
    }
    for j in 2..=n {
        for i in j..=m {
            if j == i {
                dp[j][i] = dp[j - 1][i - 1] + dd[j] * cc[i];
            } else {
                dp[j][i] = min(dp[j - 1][i - 1] + dd[j] * cc[i], dp[j][i - 1]);
            }
        }
    }

    let mut ans = std::usize::MAX;
    for i in n..=m {
        ans = min(ans, dp[n][i]);
    }
    println!("{}", ans);
}
