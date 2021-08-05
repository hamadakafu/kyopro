use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;
fn alphabet2idx(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as u8 as usize - 'a' as u8 as usize
    } else if c.is_ascii_uppercase() {
        c as u8 as usize - 'A' as u8 as usize
    } else {
        panic!("wtf")
    }
}

fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let mut toi: Vec<usize> = vec![];
    for _ in 0..n {
        let ab: (usize, usize) = parse_line().unwrap();
        toi.push(ab.1);
        toi.push(ab.0 - ab.1);
    }
    toi.sort_by(|a, b| b.cmp(&a));

    let mut ans = 0;
    for i in 0..k {
        ans += toi[i];
    }
    println!("{}", ans);
}
