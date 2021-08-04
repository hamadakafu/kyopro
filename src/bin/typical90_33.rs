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
    let (h, w): (usize, usize) = parse_line().unwrap();
    let mut ans = 0;
    if h == 1 {
        println!("{}", w);
        return;
    } else if w == 1 {
        println!("{}", h);

        return;
    }

    for h in 0..h {
        if h % 2 == 0 {
            ans += w / 2 + (w % 2);
        }
    }
    println!("{}", ans);
}
