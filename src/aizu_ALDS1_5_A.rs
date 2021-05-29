use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let aa: Vec<u64> = parse_line().unwrap();
    let q: usize = parse_line().unwrap();
    let mm: Vec<u64> = parse_line().unwrap();

    for m in mm.iter() {
        let mut continueflag = false;
        for mut bits in 0..2_u64.pow(aa.len() as u32) {
            let mut vv = vec![];
            for i in 0..n {
                if bits % 2 == 1 {
                    vv.push(aa[i]);
                }
                bits /= 2;
            }

            let s: u64 = vv.iter().sum();
            if m == &s {
                println!("yes");
                continueflag = true;
                break;
            }
        }
        if continueflag {
            continue;
        }
        println!("no");
    }
}

