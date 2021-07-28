use std::cmp::{min, max};
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
    let n: usize = parse_line().unwrap();
    let mut aa: Vec<usize> = parse_line().unwrap();
    let mut bb: Vec<usize> = parse_line().unwrap();
    aa.sort();
    bb.sort();
    let mut ans = 0;
    for (a, b) in aa.into_iter().zip(bb.into_iter()) {
        ans += (a as isize - b as isize).abs();
    }
    println!("{}", ans);
}
