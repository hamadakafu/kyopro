use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;
// dp
// https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_d
fn main() {
    let n: usize = parse_line().unwrap();
    let nums: Vec<usize> = parse_line().unwrap();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 901]; n - 1];

    dp[0][nums[0]] = 1;
    for i in 1..n - 1 {
        for j in 0..901 {
            if j >= nums[i] {
                dp[i][j] += dp[i - 1][j - nums[i]];
            }
            if j + nums[i] <= 20 {
                dp[i][j] += dp[i - 1][j + nums[i]];
            }
        }
    }
    println!("{}", dp[n - 2][nums[n - 1]]);
}
