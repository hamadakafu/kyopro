use std::cmp::{min, max};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut aa: Vec<u64> = vec![];
    let mut bb: Vec<u64> = vec![];
    let mut c: u64 = 0;
    for _ in 0..n {
        let ab: (u64, u64) = parse_line().unwrap();
        aa.push(ab.0);
        bb.push(ab.1);
        c += (ab.0 as i64 - ab.1 as i64).abs() as u64;
    }

    aa.sort();
    bb.sort();
    if n % 2 == 1 {
        let start = aa[n / 2];
        let end = bb[n / 2];
        for i in 0..n {
            c += (start as i64 - aa[i] as i64).abs() as u64;
            c += (end as i64 - bb[i] as i64).abs() as u64;
        }
    } else {
        let start1 = aa[n / 2];
        let end1 = bb[n / 2];
        let start2 = aa[n / 2] - 1 ;
        let end2 = bb[n / 2] - 1;

        let mut startd1 = 0;
        let mut startd2 = 0;
        let mut endd1 = 0;
        let mut endd2 = 0;
        for i in 0..n {
            startd1 += (start1 as i64 - aa[i] as i64).abs() as u64;
            endd1 += (end1 as i64 - bb[i] as i64).abs() as u64;
            startd2 += (start2 as i64 - aa[i] as i64).abs() as u64;
            endd2 += (end2 as i64 - bb[i] as i64).abs() as u64;
        }
        c += min(startd1, startd2);
        c += min(endd1, endd2);
    }
    println!("{}", c);
}
