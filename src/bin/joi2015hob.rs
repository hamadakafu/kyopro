use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

// 区間dp
// https://atcoder.jp/contests/joi2015ho/tasks/joi2015ho_b
// [left, right]をケーキの時計回りでの区間とし，
// その区間でどれだけjoiくんがたくさんのケーキをもらえるか
// どのような手順でケーキを選ぼうとある区間[l, r]での最適解は決まっており，
// 漸化式で書くことが出来るのでdpを使える．
// 区間dpはメモが埋まる順番が独特なので再帰関数で記述するのが良い

fn main() {
    let n: usize = parse_line().unwrap();
    let mut pieces: Vec<usize> = vec![];
    for _ in 0..n {
        pieces.push(parse_line().unwrap());
    }

    let mut memo: Vec<Vec<usize>> = vec![vec![0; n]; n];
    let mut ans = 0;
    if n == 1 {
        println!("{}", pieces[0]);
        return;
    }
    if n == 2 {
        println!("{}", max(pieces[0], pieces[1]));
        return;
    }
    for first in 0..n {
        let tmp = if pieces[(first + 1) % n] > pieces[(first + n - 1) % n] {
            dp(n, &mut memo, &pieces, (first + 2) % n, (first + n - 1) % n)
        } else {
            dp(n, &mut memo, &pieces, (first + 1) % n, (first + n - 2) % n)
        };
        ans = max(ans, tmp + pieces[first]);
    }
    println!("{}", ans);
}

/// [left, right]のケーキが残っているときにもらえる最大のケーキポイント
fn dp(n: usize, memo: &mut Vec<Vec<usize>>, pieces: &[usize], left: usize, right: usize) -> usize {
    if memo[left][right] != 0 {
        return memo[left][right];
    }
    if right >= left {
        if right - left == 1 {
            memo[left][right] = max(pieces[left], pieces[right]);
            return memo[left][right];
        } else if right - left == 0 {
            memo[left][right] = pieces[left];
            return memo[left][right];
        }
    } else {
        if n + right - left == 1 {
            memo[left][right] = max(pieces[left], pieces[right]);
            return memo[left][right];
        } else if n + right - left == 0 {
            memo[left][right] = pieces[left];
            return memo[left][right];
        }
    }

    // leftとった場合
    // ioiちゃんがleft+1取るとき
    let l = if pieces[(left + 1) % n] > pieces[right] {
        dp(n, memo, &pieces, (left + 2) % n, right)
    } else {
        dp(n, memo, &pieces, (left + 1) % n, (right + n - 1) % n)
    } + pieces[left];

    // rightとった場合
    let r = if pieces[left % n] > pieces[(right + n - 1) % n] {
        dp(n, memo, &pieces, (left + 1) % n, (right + n - 1) % n)
    } else {
        dp(n, memo, &pieces, left, (right + n - 2) % n)
    } + pieces[right];

    memo[left][right] = max(l, r);
    memo[left][right]
}
