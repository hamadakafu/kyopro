#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let mut map: Vec<Vec<usize>> = vec![];
    for _ in 0..h {
        map.push(parse_line().unwrap());
    }

    let mut ans = std::usize::MIN;
    for mut bits in 1..2_u64.pow(map.len() as u32) {
        let mut vv = vec![];
        for i in 0..map.len() {
            if bits % 2 == 1 {
                vv.push(&map[i]);
            }
            bits /= 2;
        }
        let hi = vv.len();
        // filter
        let tmp: Vec<usize> = tench(vv)
            .into_iter()
            .filter(|i| i.iter().all_equal())
            .map(|i| i[0])
            .collect();

        let mut count: HashMap<usize, usize> = HashMap::new();
        for t in tmp {
            let e = count.entry(t).or_default();
            *e += 1;
        }
        let mut maxv = std::usize::MIN;
        for (k, v) in count.iter() {
            maxv = maxv.max(*v);
        }
        ans = ans.max(hi * maxv);
    }
    println!("{}", ans);
}
fn tench(v: Vec<&Vec<usize>>) -> Vec<Vec<usize>> {
    if v.len() == 0 {
        panic!("wtf");
    }
    let h = v.len();
    let w = v[0].len();
    let mut ans = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            ans[j][i] = v[i][j];
        }
    }
    ans
}
