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
    let (h, w, n): (usize, usize, usize) = parse_line().unwrap();
    let mut pp: Vec<(usize, usize, usize)> = vec![];

    for i in 1..=n {
        let ab: (usize, usize) = parse_line().unwrap();
        pp.push((i, ab.0, ab.1));
    }

    let mut ans: Vec<(usize, usize)> = vec![(0, 0); n + 1];
    pp.sort_by(|a, b| a.1.cmp(&b.1));

    let mut tatenow = 0;
    let mut tatecount = 0;
    for p in pp.iter() {
        if tatecount != p.1 {
            tatenow += 1;
            ans[p.0].0 = tatenow;
            tatecount = p.1;
        } else {
            ans[p.0].0 = tatenow;
        }
    }
    pp.sort_by(|a, b| a.2.cmp(&b.2));
    let mut yokonow = 0;
    let mut yokocount = 0;
    for p in pp.iter() {
        if yokocount != p.2 {
            yokonow += 1;
            ans[p.0].1 = yokonow;
            yokocount = p.2;
        } else {
            ans[p.0].1 = yokonow;
        }
    }

    for a in ans.iter().skip(1) {
        println!("{} {}", a.0, a.1);
    }
}
