use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();
    let mut aa_sort = vec![];
    for (i, a) in aa.into_iter().enumerate() {
        aa_sort.push((i, a, 0));
    }

    aa_sort.sort_by(|a, b| a.1.cmp(&b.1));

    let all: usize = k / n;
    for i in 0..n {
        aa_sort[i].2 += all;
    }
    let left: usize = k % n;
    for i in 0..left {
        aa_sort[i].2 += 1;
    }

    aa_sort.sort_by(|a, b| a.0.cmp(&b.0));
    for i in 0..n {
        println!("{}", aa_sort[i].2);
    }
}
