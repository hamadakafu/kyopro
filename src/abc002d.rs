use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;
fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();

    let mut mm: HashSet<(usize, usize)> = HashSet::new();
    for _ in 0..m {
        let p = parse_line().unwrap();
        mm.insert(p);
    }

    let giins = (1..=n).collect_vec();
    let mut ans = 1;
    for mut bits in 0..2_u64.pow(giins.len() as u32) {
        let mut vv = vec![];
        for i in 0..n {
            if bits % 2 == 1 {
                vv.push(giins[i]);
            }
            bits /= 2;
        }
        if mitasu(&mm, &vv) {
            ans = max(ans, vv.len());
        }
    }
    println!("{}", ans);
}

fn mitasu(mm: &HashSet<(usize, usize)>, giin: &Vec<usize>) -> bool {
    for i in 0..giin.len() {
        for j in i+1..giin.len() {
            if !(mm.contains(&(giin[i], giin[j])) || mm.contains(&(giin[j], giin[i]))) {
                return false;
            }
        }
    }
    return true;
}
