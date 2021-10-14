#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let k: usize = parse_line().unwrap();
    if k < 10 {
        println!("{}", k);
        return;
    }
    let mut juni = 9;
    let mut now: isize = 0;
    let mut ooo = vec![];
    for i in 1..10 {
        ooo.push((-1, i));
    }
    loop {
        let t = ooo[now as usize];
        if t.1 > 0 {
            ooo.push((now, t.1 - 1));
            juni += 1;
        }
        if juni == k {
            break;
        }
        ooo.push((now, t.1));
        juni += 1;
        if juni == k {
            break;
        }
        if t.1 < 9 {
            ooo.push((now, t.1 + 1));
            juni += 1;
        }
        if juni == k {
            break;
        }
        now += 1;
    }
    let mut ans = vec![];
    let mut t = *ooo.last().unwrap();
    while t.0 != -1 {
        let pre = t.0;
        ans.push(t.1.to_string().chars().last().unwrap());
        t = ooo[pre as usize];
    }
    ans.push(t.1.to_string().chars().last().unwrap());
    println!("{}", ans.into_iter().rev().collect::<String>());
}

