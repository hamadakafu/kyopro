use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

/// しゃくとり法，尺取り法
/// ```
/// let a = 1;
/// ```
fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();

    let mut ans = 0;
    let mut start = 0;
    let mut end = 0;

    let mut ans = 1;
    let mut now_map: HashMap<usize, usize> = HashMap::new();
    let e = now_map.entry(aa[start]).or_default();
    *e += 1;
    while start != n - 1 {
        if end < n - 1 && (now_map.len() < k || now_map.contains_key(&aa[end + 1])) {
            end += 1;
            let e = now_map.entry(aa[end]).or_default();
            *e += 1;
        } else {
            if *now_map.get(&aa[start]).unwrap() == 1 {
                now_map.remove(&aa[start]);
            } else {
                let e = now_map.entry(aa[start]).or_default();
                *e -= 1;
            }
            start += 1;
        }
        ans = max(ans, end + 1 - start);
    }
    println!("{}", ans);
}
