use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (a, b, k): (u64, u64, u64) = parse_line().unwrap();

    let n = (a + b) as usize;
    let mut ans = "".to_string();

    let mut left_a = a;
    let mut left_b = b;
    let mut now_rank = 0;
    for _ in 0..a + b {
        // dbg!(left_a, left_b);
        if left_a == 0 {
            ans.push('b');
            left_b -= 1;
            continue;
        }
        if left_b == 0 {
            ans.push('a');
            left_a -= 1;
            continue;
        }
        // aを選択したときの自由度
        let mut tmp_a = nCr(left_a + left_b - 1, left_a - 1) as u64;
        if tmp_a  == 0 {
            tmp_a += 1;
        }
        let tmp_b = nCr(left_a + left_b - 1, left_a) as u64;
        let m = max(tmp_a, tmp_b);
        // dbg!(tmp_a, tmp_b, now_rank);
        if now_rank + tmp_a >= k {
            ans.push('a');
            left_a -= 1;
        } else {
            ans.push('b');
            left_b -= 1;
            now_rank += tmp_a;
        }
    }

    println!("{}", ans);
}

/// 極力u64を超えないようにnCrを計算する
fn nCr(n: u64, r: u64) -> u128 {
    if n < r {
        panic!("cant n < r, {}C{}", n, r);
    }
    if r == 0 {
        return 0;
    }

    let mut ans: u128 = 1;
    if r > n - r {
        let mut bunbo = (1..=n - r).rev().collect::<Vec<u64>>();
        for i in r + 1..=n {
            ans *= i as u128;
            if let Some(last) = bunbo.get(bunbo.len() - 1) {
                if ans % *last as u128 == 0 {
                    ans /= *last as u128;
                    bunbo.pop();
                }
            }
        }
    } else {
        let mut bunbo = (1..=r).rev().collect::<Vec<u64>>();
        for i in n - r + 1..=n {
            ans *= i as u128;
            if let Some(last) = bunbo.get(bunbo.len() - 1) {
                if ans % *last as u128 == 0 {
                    ans /= *last as u128;
                    bunbo.pop();
                }
            }
        }
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nCr() {
        assert_eq!(nCr(5, 2), 10);
        assert_eq!(nCr(5, 3), 10);
        assert_eq!(nCr(1, 1), 1);
        assert_eq!(nCr(5, 1), 5);
        assert_eq!(nCr(5, 0), 0);
        assert_eq!(nCr(2_u64 * 100000, 2), 19999900000);
    }
}
