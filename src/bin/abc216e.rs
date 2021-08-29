use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (n, mut k): (usize, usize) = parse_line().unwrap();
    let mut aa: Vec<usize> = parse_line().unwrap();
    aa.sort();

    let mut ans = 0;
    let mut duplicate = 1;
    for i in (0..n - 1).rev() {
        if (aa[i + 1] - aa[i]) * duplicate <= k {
            k -= (aa[i + 1] - aa[i]) * duplicate;
            ans += (aa[i + 1] - aa[i]) * (aa[i + 1] + aa[i] + 1) / 2 * duplicate;
            duplicate += 1;
        } else {
            let kaisuu = k / duplicate;
            // dbg!(kaisuu, k, duplicate);
            ans += duplicate * kaisuu * (aa[i + 1] + aa[i + 1] - kaisuu + 1) / 2;
            k -= duplicate * kaisuu;
            ans += k * (aa[i + 1] - kaisuu);
            k = 0;
            break;
        }
        // dbg!(ans);
    }
    if k > 0 {
        if k > aa[0] * aa.len() {
            k = aa[0] * aa.len();
        }
        let kaisuu = k / duplicate;
        ans += duplicate * kaisuu * (aa[0] + aa[0] - kaisuu + 1) / 2;
        k -= duplicate * kaisuu;
        ans += k * (aa[0] - kaisuu);
    }
    // dbg!(ans, &aa);
    println!("{}", ans);
}
