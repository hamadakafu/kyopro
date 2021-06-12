use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut kk: Vec<u64> = vec![];
    let mut sss: Vec<Vec<u64>> = vec![];
    for _ in 0..m {
        let (k, ss): (u64, Vec<u64>) = parse_line().unwrap();
        kk.push(k);
        sss.push(ss);
    }
    let pp: Vec<u64> = parse_line().unwrap();

    let mut ans = 0;
    let denkyus: Vec<u64> = (1..=n as u64).collect_vec();
    for mut bits in 0..2_u64.pow(denkyus.len() as u32) {
        let mut vv = vec![];
        for i in 0..n {
            if bits % 2 == 1 {
                vv.push(denkyus[i]);
            }
            bits /= 2;
        }
        let mut dameflag = false;
        for i in 0..m {
            let mut on_count = 0;
            for o in sss[i].iter() {
                if vv.contains(o) {
                    on_count += 1;
                }
            }
            if on_count % 2 == pp[i] {
                continue;
            } else {
                dameflag = true;
                break;
            }
        }

        if dameflag {
        } else {
            ans += 1
        }
    }
    println!("{}", ans);
}
