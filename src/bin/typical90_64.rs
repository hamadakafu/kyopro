use std::cmp::Reverse;
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
    let (n, q): (usize, usize) = parse_line().unwrap();
    let mut aa: Vec<isize> = parse_line().unwrap();
    let mut sabun: Vec<isize> = vec![aa[0]];
    for i in 1..n {
        sabun.push(aa[i] - aa[i - 1]);
    }

    let mut sabuntotal = sabun
        .iter()
        .skip(1)
        .fold(0, |acc, x| acc + x.abs());
    // dbg!(&sabun);
    for _ in 0..q {
        let (l, r, v): (usize, usize, isize) = parse_line().unwrap();
        if l != 1 {
            sabuntotal += (sabun[l - 1] + v).abs() - sabun[l - 1].abs();
        }
        sabun[l - 1] += v;

        if r != n {
            sabuntotal += (sabun[r] - v).abs() - sabun[r].abs();
            sabun[r] -= v;
        }
        // dbg!(&sabun);
        println!("{}", sabuntotal);
    }
}
