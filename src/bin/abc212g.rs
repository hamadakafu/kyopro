use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;
const mmm: usize = 998244353;
fn alphabet2idx(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as u8 as usize - 'a' as u8 as usize
    } else if c.is_ascii_uppercase() {
        c as u8 as usize - 'A' as u8 as usize
    } else {
        panic!("wtf")
    }
}

// an === b mod (p - 1) に帰着し
// (a,b)の組み合わせはaが生成する巡回群の位数の和を求めることに等しい
// 巡回群の位数の可能性は約数のみ(ラグランジュの定理)
// 効率よく計算する必要がある
fn main() {
    let p: usize = parse_line().unwrap();
    let mut gg: Vec<usize> = vec![];
    let mut n: usize = p - 1;
    for i in 1..(p as f64).sqrt() as usize + 1 {
        if n % i == 0 {
            gg.push(i);
            if i * i != n {
                gg.push(n / i);
            }
        }
    }
    gg.sort();

    let mut ff: Vec<(usize, usize)> = vec![];

    for g in gg.iter().rev() {
        let mut tmp = (p - 1) / g;
        for (preg, prenum) in ff.iter() {
            if preg % g == 0 {
                tmp -= prenum;
            }
        }
        tmp %= mmm;
        ff.push((*g, tmp));
    }

    let mut ans = 1;
    for (i, g) in gg.into_iter().rev().enumerate() {
        let mut tmp = ((p - 1) / g) % mmm;
        tmp *= ff[i].1;
        tmp %= mmm;
        ans += tmp;
        ans %= mmm;
    }
    println!("{}", ans);
}
