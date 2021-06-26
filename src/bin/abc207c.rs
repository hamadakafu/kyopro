use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut kukans: Vec<(f64, f64)> = vec![];
    for _ in 0..n {
        let (t, mut l, mut r): (usize, f64, f64) = parse_line().unwrap();
        match t {
            1 => {}
            2 => {
                r -= 0.1;
            }
            3 => {
                l += 0.1;
            }
            4 => {
                l += 0.1;
                r -= 0.1;
            }
            _ => {}
        }
        kukans.push((l, r));
    }

    kukans.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // dbg!(&kukans);

    let mut ans = 0;
    for i in 0..n {
        let target = kukans[i];
        let mut plus1idx = lower_bound(&kukans, target.1 + 0.1);
        dbg!(plus1idx, i);
        if plus1idx > 0 {
            plus1idx -= 1;
        }
        if plus1idx > i {
            ans += plus1idx - i;
        }
    }
    println!("{}", ans);
}

fn lower_bound(kukans: &Vec<(f64, f64)>, value: f64) -> usize {
    let mut left = 0;
    let mut right = kukans.len();
    while left < right {
        let pivot = (right + left) / 2;
        if value <= kukans[pivot].0 {
            right = pivot;
        } else {
            left = pivot + 1;
        }
    }
    return left;
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
