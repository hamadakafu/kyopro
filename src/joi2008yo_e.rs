use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (r, c): (usize, usize) = parse_line().unwrap();

    let mut senbeis: Vec<Vec<bool>> = vec![];
    for _ in 0..r {
        let line: Vec<usize> = parse_line().unwrap();
        let line = line.into_iter().map(|b| b == 0).collect_vec();
        senbeis.push(line);
    }

    let gyous = (0..r).collect_vec();
    let mut ans = 0;
    for mut bits in 0..2_u64.pow(gyous.len() as u32) {
        let mut vv = vec![];
        for i in 0..gyous.len() {
            if bits % 2 == 1 {
                vv.push(gyous[i]);
            }
            bits /= 2;
        }

        ans = max(ans, nanko(senbeis.clone(), &vv));
    }
    println!("{}", ans);
}

fn nanko(mut senbeis: Vec<Vec<bool>>, gyous: &Vec<usize>) -> u64 {
    for r in gyous.iter() {
        senbeis[*r] = senbeis[*r].iter().map(|b| !b).collect_vec();
    }

    let mut ans = 0;
    for c in 0..senbeis[0].len() {
        let mut trus = 0;
        for r in 0..senbeis.len() {
            if senbeis[r][c] {
                trus += 1;
            }
        }
        ans += max(senbeis.len() as u64 - trus, trus);
    }
    return ans;
}
