use itertools::Itertools;
use std::cmp::{max, min};
use whiteread::parse_line;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut hashiras_vec: Vec<(i64, i64)> = vec![];
    let mut hashiras_set: HashSet<(i64, i64)> = HashSet::new();

    for _ in 0..n {
        let hashira: Vec<i64> = parse_line().unwrap();
        hashiras_vec.push((hashira[0], hashira[1]));
        hashiras_set.insert((hashira[0], hashira[1]));
    }

    let mut ans = 0;

    for h in hashiras_vec.into_iter().combinations(2) {
        let (left, right): ((i64, i64), (i64, i64)) = {
            if h[0].0 > h[1].0 {
                ((h[1].0, h[1].1), (h[0].0, h[0].1))
            } else {
                ((h[0].0, h[0].1), (h[1].0, h[1].1))
            }
        };

        let disx = right.0 as i64 - left.0 as i64;
        let disy = right.1 as i64 - left.1 as i64;

        let onex = left.0 as i64 - disy;
        let oney = left.1 as i64 + disx;
        let twox = onex + disx;
        let twoy = oney + disy;
        if hashiras_set.contains(&(onex, oney)) && hashiras_set.contains(&(twox, twoy)) {
            ans = max(ans, disx.pow(2) + disy.pow(2));
            continue;
        }

        let onex = left.0 as i64 + disy;
        let oney = left.1 as i64 - disx;
        let twox = onex + disx;
        let twoy = oney + disy;
        if hashiras_set.contains(&(onex, oney)) && hashiras_set.contains(&(twox, twoy)) {
            ans = max(ans, disx.pow(2) + disy.pow(2));
        }
    }

    println!("{}", ans);
}
