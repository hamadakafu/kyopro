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
    let n: usize = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();
    let bb: Vec<usize> = parse_line().unwrap();
    let cc: Vec<usize> = parse_line().unwrap();
    let mut aamod: Vec<usize> = vec![0; 46];
    let mut bbmod: Vec<usize> = vec![0; 46];
    let mut ccmod: Vec<usize> = vec![0; 46];
    for a in aa {
        aamod[a % 46] += 1;
    }
    for b in bb {
        bbmod[b % 46] += 1;
    }
    for c in cc {
        ccmod[c % 46] += 1;
    }

    let mut ans = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += aamod[i] * bbmod[j] * ccmod[k];
                }
            }
        }
    }
    println!("{}", ans);
}
