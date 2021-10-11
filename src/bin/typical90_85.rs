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
    let yakusu = yakusurekkyo(k);
    let mut ans = 0;
    for a in 0..yakusu.len() {
        for b in a..yakusu.len() {
            let tmp = k / yakusu[a] / yakusu[b];
            if tmp * yakusu[a] * yakusu[b] == k && yakusu[b] <= tmp {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

/// O(sqrt(N) + sqrt(N) * log(sqrt(N)))
fn yakusurekkyo(n: usize) -> Vec<usize> {
    let mut i = 1;
    let mut ans = vec![];
    while i * i <= n {
        if n % i == 0 {
            ans.push(i);
            let tmp = n / i;
            if i != tmp {
                // かぶりをなくす
                ans.push(tmp);
            }
        }
        i += 1;
    }
    ans.sort();
    ans
}
