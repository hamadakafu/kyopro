use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let n: usize = parse_line().unwrap();
    let ss: Vec<usize> = parse_line().unwrap();
    let tt: Vec<usize> = parse_line().unwrap();

    let mut junban: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();

    for (i, t) in tt.iter().enumerate() {
        junban.push((Reverse(*t), i + 1));
    }

    let mut ans: Vec<usize> = vec![std::usize::MAX; n + 1];
    let mut already: Vec<bool> = vec![false; n + 1];

    while !junban.is_empty() {
        let (Reverse(t), i) = junban.pop().unwrap();
        ans[i] = min(ans[i], t);
        already[i] = true;
        let ni = if i == n { 1 } else { i + 1 };
        if already[ni] {
            continue;
        }
        junban.push((Reverse(ans[i] + ss[i - 1]), ni));
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
