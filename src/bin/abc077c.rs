use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut aa: Vec<u64> = parse_line().unwrap();
    let mut bb: Vec<u64> = parse_line().unwrap();
    let mut cc: Vec<u64> = parse_line().unwrap();

    aa.sort();
    bb.sort();
    cc.sort();

    let mut btoc = vec![];
    let mut btoa = vec![];

    let mut ans = 0;

    for (bi, b) in bb.iter().enumerate() {
        let ci = cc.upper_bound(*b);
        if ci >= n {
            btoc.push(0);
        } else {
            btoc.push(n - ci);
        }
        let ai = aa.lower_bound(*b);

        if ai == 0 {
            btoa.push(0);
        } else {
            btoa.push(ai);
        }
    }

    for i in 0..n {
        ans += btoc[i] * btoa[i];
    }
    println!("{}", ans);
}

trait LUBound<T> {
    fn lower_bound(&self, value: T) -> usize
    where
        T: PartialOrd;
    fn upper_bound(&self, value: T) -> usize
    where
        T: PartialOrd;
}
impl<T> LUBound<T> for Vec<T> {
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
        let cases = vec![
            (
                vec![0, 1, 1, 1, 2, 2, 3, 5],
                (-1..=6),
                vec![0, 0, 1, 4, 6, 7, 7, 8],
            ),
            (vec![1, 5], (2..=3), vec![1, 1]),
        ];
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
