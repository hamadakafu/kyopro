#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::{parse_line, parse_string};

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let mut n: usize = parse_line().unwrap();

    let mut ans: usize = std::usize::MIN;
    let mut leftnode = 1;
    for i in 1..=n {
        println!("? 1 {}", i);
        let tmp: usize = parse_line().unwrap();
        if ans < tmp {
            ans = tmp;
            leftnode = i;
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        println!("? {} {}", leftnode, i);
        let tmp: usize = parse_line().unwrap();
        if ans < tmp {
            ans = tmp;
        }
    }
    println!("! {}", ans);
}
