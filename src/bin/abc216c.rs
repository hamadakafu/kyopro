use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let mut n: usize = parse_line().unwrap();
    let mut ans = vec![];
    while n > 0 {
        if n % 2 == 1 {
            ans.push('A');
            n -= 1;
        } else {
            ans.push('B');
            n /= 2;
        }
    }
    for c in ans.into_iter().rev() {
        print!("{}", c);
    }
    println!();
}
