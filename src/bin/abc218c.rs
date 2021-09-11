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
    let n: usize = parse_line().unwrap();
    let mut sss: Vec<Vec<char>> = vec![];
    for i in 0..n {
        let ss: String = parse_line().unwrap();
        sss.push(ss.chars().collect_vec());
    }
    let mut ttt: Vec<Vec<char>> = vec![];
    for i in 0..n {
        let tt: String = parse_line().unwrap();
        ttt.push(tt.chars().collect_vec());
    }
    let ttt = trimtopleft(&ttt);
    if trimtopleft(&sss) == ttt {
        println!("Yes");
        return;
    }
    if trimtopleft(&rotate(&sss, (true, false))) == ttt {
        println!("Yes");
        return;
    }
    if trimtopleft(&rotate(&sss, (false, false))) == ttt {
        println!("Yes");
        return;
    }
    if trimtopleft(&rotate(&sss, (false, true))) == ttt {
        println!("Yes");
        return;
    }
    println!("No");
}

/// 正方形を仮定している2次元配列の回転
fn rotate<T>(sss: &Vec<Vec<T>>, kakudo: (bool, bool)) -> Vec<Vec<T>>
where
    T: Clone + Default,
{
    let n = sss.len();
    match kakudo {
        (true, true) => sss.clone(),
        // 90
        (true, false) => {
            let mut new = vec![vec![T::default(); n]; n];
            for (i, ss) in sss.iter().enumerate() {
                for (j, s) in ss.iter().enumerate() {
                    new[n - 1 - j][i] = s.clone();
                }
            }
            new
        }
        // 180
        (false, true) => {
            let mut new = vec![vec![T::default(); n]; n];
            for (i, ss) in sss.iter().enumerate() {
                for (j, s) in ss.iter().enumerate() {
                    new[n - 1 - i][n - 1 - j] = s.clone();
                }
            }
            new
        }
        // 270
        (false, false) => {
            let mut new = vec![vec![T::default(); n]; n];
            for (i, ss) in sss.iter().enumerate() {
                for (j, s) in ss.iter().enumerate() {
                    new[j][n - 1 - i] = s.clone();
                }
            }
            new
        }
    }
}

fn trimtopleft(sss: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut s_min_x: usize = std::usize::MAX;
    let mut s_max_x: usize = 0;
    let mut s_min_y: usize = std::usize::MAX;
    let mut s_max_y: usize = 0;
    for (x, ss) in sss.iter().enumerate() {
        for (y, &s) in ss.iter().enumerate() {
            if s == '#' {
                s_min_y = min(s_min_y, y);
                s_min_x = min(s_min_x, x);
                s_max_y = max(s_max_y, y);
            }
        }
        if ss.iter().any(|&c| c == '#') {
            s_max_x = max(s_max_x, x);
        }
    }
    let mut new = vec![];
    let n = sss.len();
    for i in s_min_x..n {
        let mut tmp = vec![];
        for &s in sss[i].iter().skip(s_min_y) {
            tmp.push(s);
        }
        for _ in 0..s_min_y {
            tmp.push('.');
        }
        new.push(tmp);
    }
    for _ in 0..s_min_x {
        new.push(vec!['.'; n]);
    }
    new
}
