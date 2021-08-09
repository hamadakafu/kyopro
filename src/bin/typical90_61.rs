use std::cmp::{min, max};
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Reverse;

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
    let q: usize = parse_line().unwrap();
    let mut cards: VecDeque<usize> = VecDeque::new();
    for _ in 0..q {
        let (t, x): (usize, usize) = parse_line().unwrap();
        if t == 1 {
            cards.push_front(x);
        } else if t == 2 {
            cards.push_back(x);
        } else if t == 3 {
            println!("{}", cards[x - 1]);
        }
    }
}
