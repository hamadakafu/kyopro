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
    let n: usize = parse_line().unwrap();
    let mut nichis: HashMap<usize, isize> = HashMap::new();
    for _ in 0..n {
        let (a, b): (usize, usize) = parse_line().unwrap();
        let e = nichis.entry(a).or_default();
        *e += 1;
        let e = nichis.entry(a + b).or_default();
        *e += -1;
    }
    // dbg!(&nichis);
    let mut logins: Vec<isize> = vec![];
    let mut hi = nichis.keys().collect_vec();
    hi.sort();
    // dbg!(&hi);

    let mut now = 0;
    for ni in nichis.iter().sorted() {
        now += ni.1;
        logins.push(now);
    }

    // dbg!(&logins);
    let mut ans = vec![0; n + 1];
    for i in 0..logins.len() - 1 {
        let l = logins[i];
        ans[l as usize] += hi[i + 1] - hi[i];
    }
    for i in 1..ans.len() - 1 {
        print!("{} ", ans[i]);
    }
    println!("{}", ans.last().unwrap());
}
