use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let pp: Vec<u64> = parse_line().unwrap();
    let qq: Vec<u64> = parse_line().unwrap();

    let mut a = 0;
    let mut b = 0;
    for (i, num) in (1..=n as u64).into_iter().permutations(n).enumerate() {
        if pp == num {
            a = i as i64 + 1;
        }
        if qq == num {
            b = i as i64 + 1;
        }
    }
    println!("{}", (a - b).abs());
}
