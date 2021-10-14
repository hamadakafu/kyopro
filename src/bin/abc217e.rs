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

// 逐次的に処理
// priority queueとvecdequeuをうまく使うことでオンライン(online)で出力できる．
fn main() {
    let q: usize = parse_line().unwrap();
    let mut pq: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut tmp = VecDeque::new();
    for _ in 0..q {
        let query: Vec<usize> = parse_line().unwrap();

        if query.len() == 2 {
            let x = query[1];
            tmp.push_back(Reverse(x));
        } else if query[0] == 2 {
            if let Some(v) = pq.pop() {
                println!("{}", v.0);
            } else {
                println!("{}", tmp.pop_front().unwrap().0);
            }
        } else {
            for t in tmp {
                pq.push(t);
            }
            tmp = VecDeque::new();
        }
    }
}
