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
    let n: usize = parse_line().unwrap();
    let mut aa: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        aa.push(parse_line().unwrap());
    }

    let mut ans = 1;
    for i in 0..n {
        let tmp: usize = ans;
        let mut newans = 0;
        for a in aa[i].iter() {
            newans += tmp * a;
            newans %= ten97;
        }
        ans = newans;
    }
    println!("{}", ans);
}
