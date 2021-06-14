use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, m, k): (usize, usize, usize) = parse_line().unwrap();

    if n <= k {
        // すべてのパターンok
    }

    for i in 1..=n + m {
        if i <= k {
            continue;
        }

        nCr(i as u64 - 1 , k as u64);
        let wi = k + 1;
        let bi = wi - wi;
        n - k - 1;

    }
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
