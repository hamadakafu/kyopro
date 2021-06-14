use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut aa: Vec<usize> = parse_line().unwrap();
    aa.sort();
    if (1..=n).collect_vec() == aa {
        println!("Yes");
    } else {
        println!("No");
    }
}
