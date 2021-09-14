#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

// 半分全列挙
// 40は半分全列挙
// 2^20 = 1000000 なので効率良く解ける
// グループを2つに分けてそれぞれで全探索する
// 片方でk1個選びもう片方でk1 + k2 = kとなるようにk2を選ぶ．
// その際制約を満たすような個数を加算する
fn main() {
    let (n, k, p): (usize, usize, usize) = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();

    let mut g1 = vec![];
    let mut g2 = vec![];
    for (i, &a) in aa.iter().enumerate() {
        if i % 2 == 0 {
            g1.push(a);
        } else {
            g2.push(a);
        }
    }

    let mut g1tansaku: Vec<Vec<usize>> = vec![vec![]; g1.len() + 1];
    let mut g2tansaku: Vec<Vec<usize>> = vec![vec![]; g2.len() + 1];

    g1tansaku[0].push(0);
    g2tansaku[0].push(0);
    for i in 1..=g1.len() {
        for idxs in g1.iter().combinations(i) {
            let tmp: usize = idxs.iter().fold(0, |acc, x| **x + acc);
            g1tansaku[i].push(tmp);
        }
        g1tansaku[i].sort();
    }
    for i in 1..=g2.len() {
        for idxs in g2.iter().combinations(i) {
            let tmp: usize = idxs.iter().fold(0, |acc, x| acc + **x);
            g2tansaku[i].push(tmp);
        }
        g2tansaku[i].sort();
    }

    let mut ans = 0;
    let g2len = g2tansaku.len();
    for i in 0..=g1.len() {
        if i > k {
            break;
        }
        let j = k - i;
        if j >= g2len {
            continue;
        }

        for vi in g1tansaku[i].iter() {
            if *vi > p {
                break;
            }
            let vj = p - vi;
            // dbg!(i, j, vi, vj, &g2tansaku[j]);
            ans += match g2tansaku[j].binary_search(&vj) {
                Ok(a) => {
                    // dbg!(a);
                    a + 1
                }
                Err(b) => {
                    // dbg!(b);
                    b
                }
            };
        }
    }
    println!("{}", ans);
}
trait LUBound<T> {
    fn range_count(&self, start: T, end: T) -> usize
    where
        T: PartialOrd;

    fn lower_bound(&self, value: T) -> usize
    where
        T: PartialOrd;
    fn upper_bound(&self, value: T) -> usize
    where
        T: PartialOrd;
}
impl<T> LUBound<T> for Vec<T> {
    // [start, end)
    #[inline(always)]
    fn range_count(&self, start: T, end: T) -> usize
    where
        T: PartialOrd,
    {
        let li = self.lower_bound(start);
        let ri = self.lower_bound(end);
        ri - li
    }
    #[inline(always)]
    fn lower_bound(&self, value: T) -> usize
    where
        T: PartialOrd,
    {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let pivot = (right + left) / 2;
            if value <= self[pivot] {
                right = pivot;
            } else {
                left = pivot + 1;
            }
        }
        return left;
    }

    #[inline(always)]
    fn upper_bound(&self, value: T) -> usize
    where
        T: PartialOrd,
    {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let pivot = (right + left) / 2;
            if value < self[pivot] {
                right = pivot;
            } else {
                left = pivot + 1;
            }
        }
        return left;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lower_bound() {
        let cases = vec![(
            vec![0, 1, 1, 1, 2, 2, 3, 5],
            (-1..=6),
            vec![0, 0, 1, 4, 6, 7, 7, 8],
        )];
        for case in cases {
            for (value, expected) in case.1.zip(case.2.into_iter()) {
                assert_eq!(case.0.lower_bound(value), expected);
            }
        }
    }
    #[test]
    fn upper_bound() {
        let cases = vec![(
            vec![0, 1, 1, 1, 2, 2, 3, 5],
            (-1..=6),
            vec![0, 1, 4, 6, 7, 7, 8, 8],
        )];
        for case in cases {
            for (value, expected) in case.1.zip(case.2.into_iter()) {
                assert_eq!(case.0.upper_bound(value), expected);
            }
        }
    }
}
