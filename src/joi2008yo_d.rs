use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let m: usize = parse_line().unwrap();
    let mut seiza: Vec<(i64, i64)> = vec![];
    for _ in 0..m {
        seiza.push(parse_line().unwrap());
    }
    let mut kouho: Vec<(i64, i64)> = vec![];
    let n: usize = parse_line().unwrap();
    for _ in 0..n {
        let k: (i64, i64) = parse_line().unwrap();
        kouho.push(k);
    }

    // 1個目の星で正規化
    let mut seiza_seikika: Vec<(i64, i64)> = vec![];
    for i in 0..m {
        seiza_seikika.push((seiza[i].0 - seiza[0].0, seiza[i].1 - seiza[0].1));
    }

    // i番目の星で正規化
    for i in 0..n {
        let mut seikikakouho: HashSet<(i64, i64)> = HashSet::new();
        for j in 0..n {
            seikikakouho.insert((kouho[j].0 - kouho[i].0, kouho[j].1 - kouho[i].1));
        }

        let mut noflag = false;
        for s in seiza_seikika.iter() {
            if seikikakouho.contains(&s) {
                continue;
            } else {
                noflag = true;
                break;
            }
        }
        if !noflag {
            println!("{} {}", -(seiza[0].0 - kouho[i].0), -(seiza[0].1 - kouho[i].1));
            return;
        }
    }
}
