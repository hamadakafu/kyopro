use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: u128 = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (l, r): (usize, usize) = parse_line().unwrap();
    let (l, r): (u128, u128) = (l as u128, r as u128);
    let mut hajime = keta(l);
    let owari = keta(r);
    dbg!(owari, hajime);
    let mut ans = 0;
    if hajime != owari {
        let mut tmp = l + 10_u128.pow(hajime as u32) - 1;
        tmp %= ten97;
        tmp *= 10_u128.pow(hajime as u32) - l;
        tmp %= ten97;
        tmp *= inv2ten97;
        tmp %= ten97;
        tmp *= hajime;
        tmp %= ten97;
        ans += tmp;
        ans %= ten97;

        let mut tmp = 10_u128.pow(owari as u32 - 1) + r;
        tmp %= ten97;
        tmp *= r - 10_u128.pow(owari as u32 - 1) + 1;
        tmp %= ten97;
        tmp *= inv2ten97;
        tmp %= ten97;
        tmp *= owari;
        tmp %= ten97;
        ans += tmp;
        ans %= ten97;
    } else {
        let mut tmp = r + l;
        tmp %= ten97;
        tmp *= r - l + 1;
        tmp %= ten97;
        tmp *= inv2ten97;
        tmp %= ten97;
        tmp *= hajime;
        tmp %= ten97;
        ans += tmp;
        ans %= ten97;
        println!("{}", ans);
        return;
    }
    ans %= ten97;
    dbg!(ans);
    hajime += 1;
    while hajime < owari {
        let mut tmp = 10_u128.pow(hajime as u32 - 1) + 10_u128.pow(hajime as u32) - 1;
        tmp %= ten97;
        tmp *= 10_u128.pow(hajime as u32) - 10_u128.pow(hajime as u32 - 1);
        tmp %= ten97;
        tmp *= inv2ten97;
        tmp %= ten97;
        tmp *= hajime;
        tmp %= ten97;
        ans += tmp;
        ans %= ten97;
        hajime += 1;
    }
    println!("{}", ans);
}

fn keta(mut a: u128) -> u128 {
    let mut ans = 0;
    while a > 0 {
        ans += 1;
        a /= 10;
    }
    ans
}
