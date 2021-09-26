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

const mmm: usize = 998244353;
fn main() {
    let (n, d): (usize, usize) = parse_line().unwrap();
    let mut ans = 0;
    let stop = (d as f64 / 2.0).ceil() as usize;
    // dbg!(stop);
    for up in (stop..=d).rev() {
        let down = d - up;
        if n < up {
            continue;
        }
        let tmp1 = mypow(2, n - up) - 1;

        let mut tmp2 = mypow(2, up);

        let downtmp = if down == 0 || down == 1 {
            1
        } else {
            mypow(2, down - 1)
        };
        // dbg!(up, down, tmp1, tmp2, downtmp);
        tmp2 *= downtmp;
        tmp2 %= mmm;
        if up == d / 2 {
            ans += tmp1 * tmp2;
        } else {
            ans += tmp1 * tmp2;
            ans += tmp1 * tmp2;
        }
        ans %= mmm;
        // dbg!(ans);
    }
    println!("{}", ans);
}

fn mypow(a: usize, mut b: usize) -> usize {
    let mut ans = 1;
    let mut ppp = a;
    while b > 0 {
        if b % 2 == 1 {
            ans *= ppp;
            ans %= mmm;
        }
        ppp *= ppp;
        ppp %= mmm;

        b /= 2;
    }
    ans
}
