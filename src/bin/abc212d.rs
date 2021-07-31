use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::vec;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;


// すでにあるデータとの差分を保存しておく
// 取り出すときにその値をapplyする
fn main() {
    let q: usize = parse_line().unwrap();
    let mut bh = BinaryHeap::new();
    let mut diff: isize = 0;
    for _ in 0..q {
        let query: Vec<isize> = parse_line().unwrap();
        if query[0] == 1 {
            bh.push(Reverse(query[1] + diff));
        } else if query[0] == 2 {
            diff -= query[1];
        } else {
            let t = bh.pop().unwrap();
            println!("{}", t.0 - diff);
        }
    }
}
