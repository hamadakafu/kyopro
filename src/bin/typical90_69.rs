use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    if n == 1 {
        println!("{}", k);
        return;
    } else if n == 2 {
        println!("{}", k * (k - 1));
        return;
    }
    let tmp = (k * (k - 1)) % ten97;
    println!("{}", (tmp * mypow(k - 2, n - 2, ten97)) % ten97);
}

fn mypow(a: usize, mut b: usize, mmm: usize) -> usize {
    let mut ans = 1;
    let mut ppp = a;
    while b > 0 {
        if b % 2 == 1 {
            ans *= ppp;
            ans %= mmm;
        }
        ppp *= ppp;
        ppp %= mmm;

        b /= 2;
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::mypow;

    #[test]
    fn test_mypow() {
        let cases = vec![(3, 7, 5, 2), (100, 20, 231, 67)];
        for (a, b, mmm, ans) in cases {
            assert_eq!(mypow(a, b, mmm), ans);
        }
    }
}
