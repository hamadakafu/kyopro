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
    let (a, b): (usize, usize) = parse_line().unwrap();

    let tmp = lcm(a as u128, b as u128);
    if tmp > 10_u128.pow(18) {
        println!("Large");
    } else {
        println!("{}", tmp);
    }
}
fn lcm(a: u128, b: u128) -> u128 {
    let tmp = gcd(a, b);
    a * b / tmp
}

fn gcd(a: u128, b: u128) -> u128 {
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
