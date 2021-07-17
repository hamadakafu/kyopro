use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let cc: Vec<usize> = parse_line().unwrap();
    let mut m = HashMap::new();
    for i in 0..k {
        let e = m.entry(cc[i]).or_insert(0);
        *e += 1;
    }

    let mut ans = m.len();
    for i in k..n {
        let e = m.entry(cc[i - k]).or_insert(0);
        *e -= 1;
        if *e == 0 {
            m.remove_entry(&cc[i-k]);
        }
        let e = m.entry(cc[i]).or_insert(0);
        *e += 1;
        ans = max(ans, m.len());
    }

    println!("{}", ans);
}
