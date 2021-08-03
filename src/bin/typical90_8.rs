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

// 部分列の個数
// atcoder

fn main() {
    let n: usize = parse_line().unwrap();
    let s: String = parse_line().unwrap();

    let mut aa: usize = 0;
    let mut tt: usize = 0;
    let mut cc: usize = 0;
    let mut oo: usize = 0;
    let mut dd: usize = 0;
    let mut ee: usize = 0;
    let mut rr: usize = 0;
    for c in s.chars() {
        if c == 'a' {
            aa += 1;
            aa %= ten97;
        }
        if c == 't' {
            tt += aa;
            tt %= ten97;
        }
        if c == 'c' {
            cc += tt;
            cc %= ten97;
        }
        if c == 'o' {
            oo += cc;
            oo %= ten97;
        }
        if c == 'd' {
            dd += oo;
            dd %= ten97;
        }
        if c == 'e' {
            ee += dd;
            ee %= ten97;
        }
        if c == 'r' {
            rr += ee;
            rr %= ten97;
        }
    }
    println!("{}", rr);
}
