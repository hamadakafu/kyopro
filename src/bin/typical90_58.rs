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
    let (mut n, k): (usize, usize) = parse_line().unwrap();
    let mut nums: Vec<(bool, usize)> = vec![(false, 0); 100000];
    let mut count = 0;
    nums[n] = (true, count);
    if n == 0 {
        println!("0");
        return;
    }
    loop {
        // dbg!(1, count, n);
        n = (hoge(n).into_iter().sum::<usize>() + n) % 100000;
        if nums[n].0 == true {
            dbg!(n, nums[n]);
            break;
        }
        count += 1;
        nums[n] = (true, count);
        // dbg!(2, count, n);
        if count >= k {
            // dbg!(count, k);
            println!("{}", n);
            return;
        }
    }
    let k = (k - nums[n].1) % (count + 1 - nums[n].1);

    dbg!(count, k, count + 1 - nums[n].1);
    let mut count = 0;
    loop {
        if count >= k {
            println!("{}", n);
            return;
        }
        n = (hoge(n).into_iter().sum::<usize>() + n) % 100000;
        count += 1;
    }
}

fn hoge(mut n: usize) -> Vec<usize> {
    let mut ans = vec![];
    while n != 0 {
        ans.push(n % 10);
        n /= 10;
    }
    ans
}
