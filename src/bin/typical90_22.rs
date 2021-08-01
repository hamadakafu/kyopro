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

fn gcd(a: usize, b: usize) -> usize {
    if a > b {
        let tmp = a % b;
        if tmp == 0 {
            b
        } else {
            gcd(b, tmp)
        }
    } else {
        let tmp = b % a;
        if tmp == 0 {
            a
        } else {
            gcd(a, tmp)
        }
    }
}

fn main() {
    let (a, b, c): (usize, usize, usize) = parse_line().unwrap();
    let g = gcd(gcd(a, b), c);
    println!("{}", (a / g - 1) +(b / g - 1) +(c / g - 1));
}
