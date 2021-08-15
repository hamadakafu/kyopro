use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let n: usize = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();
    let mut goukei: usize = aa.iter().sum();
    if goukei % 10 == 0 {
        goukei /= 10;
    } else {
        println!("No");
        return;
    }
    let mut sumaa: Vec<usize> = vec![0];
    for i in 0..2 * n {
        sumaa.push(sumaa[i] + aa[i % n]);
    }

    for l in 0..2 * n + 1 {
        let r = sumaa.lower_bound(goukei + sumaa[l]);
        if r == 2 * n + 1 {
            continue;
        }
        if sumaa[r] - sumaa[l] == goukei {
            println!("Yes");
            return;
        }
    }
    println!("No");
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
