use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;
// nをStringで受けないとruntime errorになる
fn main() {
    let (n, k): (String, usize) = parse_line().unwrap();

    let mut ans = n;
    for _ in 0..k {
        let tmp_num = to_u128(&ans, 8);
        dbg!(tmp_num);
        let tmp = from_u128(tmp_num, 9);
        dbg!(tmp_num, &tmp);
        ans = tmp.replace("8", "5").chars().collect();
    }
    println!("{}", ans);
}

// 10進法になおす
fn to_u128(s: &str, base: u128) -> u128 {
    let mut tmp: u128 = 1;
    let mut ans: u128 = 0;
    for c in s.chars().rev() {
        ans += tmp * c.to_digit(10).unwrap() as u128;
        tmp *= base;
    }
    ans
}
fn from_u128(num: u128, base: u128) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let mut tmp: u128 = num;
    let mut ans = vec![];
    while tmp > 0 {
        let t: u128 = tmp % base;
        ans.push(std::char::from_digit(t as u32, 10).unwrap());
        tmp /= base;
    }
    ans.into_iter().rev().join("")
}

#[cfg(test)]
mod tests {
    use crate::{from_u128, to_u128};

    #[test]
    fn test_to_u128() {
        let cases = vec![("0", 8, 0), ("77", 8, 63), ("76", 8, 62), ("16", 8, 14)];
        for (s, base, ans) in cases {
            assert_eq!(to_u128(s, base), ans);
        }
    }
    #[test]
    fn test_from_u128() {
        let cases = vec![(0, 8, "0"), (63, 8, "77"), (62, 8, "76"), (14, 8, "16")];
        for (s, base, ans) in cases {
            assert_eq!(from_u128(s, base), ans);
        }
    }
}
