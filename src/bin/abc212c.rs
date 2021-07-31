use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;
fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut aa: Vec<isize> = parse_line().unwrap();
    let mut bb: Vec<isize> = parse_line().unwrap();

    aa.sort();
    bb.sort();
    let mut ai = 0;
    let mut bi = 0;
    let mut ans = (aa[ai] - bb[bi]).abs();
    loop {
        if ai == n - 1 && bi == m - 1 {
            break;
        }
        if ai == n - 1 {
            bi += 1;
            ans = min(ans, (aa[ai] - bb[bi]).abs());
        } else if bi == m - 1 {
            ai += 1;
            ans = min(ans, (aa[ai] - bb[bi]).abs());
        } else if aa[ai] > bb[bi] {
            bi += 1;
            ans = min(ans, (aa[ai] - bb[bi]).abs());
        } else if bb[bi] > aa[ai] {
            ai += 1;
            ans = min(ans, (aa[ai] - bb[bi]).abs());
        } else {
            if aa[ai + 1] > bb[bi + 1] {
                bi += 1;
                ans = min(ans, (aa[ai] - bb[bi]).abs());
            } else {
                ai += 1;
                ans = min(ans, (aa[ai] - bb[bi]).abs());
            }
        }
    }
    println!("{}", ans);
}
