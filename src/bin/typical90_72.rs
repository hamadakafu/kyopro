#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let tmp: String = parse_line().unwrap();
        map.push(tmp.chars().collect_vec());
    }

    let mut ans: isize = -1;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '.' {
                let mut already: Vec<Vec<bool>> = vec![vec![false; w]; h];
                already[i][j] = true;
                ans = ans.max(cal(h, w, 0, (i, j), (i, j), &map, already));
                // dbg!(i, j, ans);
            }
        }
    }

    println!("{}", ans);
}

fn cal(
    h: usize,
    w: usize,
    now_dis: isize,
    start: (usize, usize),
    (i, j): (usize, usize),
    map: &Vec<Vec<char>>,
    already: Vec<Vec<bool>>,
) -> isize {
    let mut tmp: isize = -1;
    if i > 0 && map[i - 1][j] == '.' {
        if start == (i - 1, j) && now_dis + 1 > 3 {
            tmp = tmp.max(now_dis + 1);
        }
        if !already[i - 1][j] {
            let mut newalready = already.clone();
            newalready[i - 1][j] = true;
            tmp = tmp.max(cal(h, w, now_dis + 1, start, (i - 1, j), map, newalready));
        }
    }
    if j > 0 && map[i][j - 1] == '.' {
        if start == (i, j - 1) && now_dis + 1 > 3 {
            tmp = tmp.max(now_dis + 1);
        }
        if !already[i][j - 1] {
            let mut newalready = already.clone();
            newalready[i][j - 1] = true;
            tmp = tmp.max(cal(h, w, now_dis + 1, start, (i, j - 1), map, newalready));
        }
    }
    if i < h - 1 && map[i + 1][j] == '.' {
        if start == (i + 1, j) && now_dis + 1 > 3 {
            tmp = tmp.max(now_dis + 1);
        }
        if !already[i + 1][j] {
            let mut newalready = already.clone();
            newalready[i + 1][j] = true;
            tmp = tmp.max(cal(h, w, now_dis + 1, start, (i + 1, j), map, newalready));
        }
    }
    if j < w - 1 && map[i][j + 1] == '.' {
        if start == (i, j + 1) && now_dis + 1 > 3 {
            tmp = tmp.max(now_dis + 1);
        }
        if !already[i][j + 1] {
            let mut newalready = already.clone();
            newalready[i][j + 1] = true;
            tmp = tmp.max(cal(h, w, now_dis + 1, start, (i, j + 1), map, newalready));
        }
    }
    tmp
}
