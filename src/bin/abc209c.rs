use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let m = 1000000000 + 7;
    let n: usize = parse_line().unwrap();
    let mut cc: Vec<usize> = parse_line().unwrap();

    cc.sort();

    let mut ans = 1;
    for (i, c) in cc.iter().enumerate() {
        if *c > i {
            ans *= c - i;
            ans %= m;
        } else {
            println!("0");
            return;
        }
    }
    println!("{}", ans);
}
