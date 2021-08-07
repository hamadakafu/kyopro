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
    let (n, l): (usize, usize) = parse_line().unwrap();
    let mut kaidan: Vec<usize> = vec![1];

    for i in 1..=n {
        let mut tmp = 0;
        if i >= l {
            tmp += kaidan[i - l];
            tmp %= ten97;
        }
        tmp += kaidan[i - 1];
        tmp %= ten97;
        kaidan.push(tmp);
    }
    println!("{}", kaidan[n]);
}
