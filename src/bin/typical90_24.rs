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
    let (n, k): (usize, isize) = parse_line().unwrap();
    let aa: Vec<isize> = parse_line().unwrap();
    let bb: Vec<isize> = parse_line().unwrap();

    let mut diff = 0;
    for i in 0..n {
        diff += (aa[i] - bb[i]).abs();
    }
    if k >= diff && (k - diff) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
