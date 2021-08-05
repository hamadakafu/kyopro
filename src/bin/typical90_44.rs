use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

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
    let aa: Vec<usize> = parse_line().unwrap();
    let mut aa: VecDeque<usize> = aa.into();
    for _ in 0..q {
        let (t, x, y): (usize, usize, usize) = parse_line().unwrap();
        if t == 1 {
            let tmp = aa[x - 1];
            aa[x - 1] = aa[y - 1];
            aa[y - 1] = tmp;
        } else if t == 2 {
            let tmp = aa.pop_back().unwrap();
            aa.push_front(tmp);
        } else if t == 3 {
            println!("{}", aa[x - 1]);
        }
    }
}
