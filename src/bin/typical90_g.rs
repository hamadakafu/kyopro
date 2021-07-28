use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;
fn alphabet2idx(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as u8 as usize - 'a' as u8 as usize
    } else if c.is_ascii_uppercase() {
        c as u8 as usize - 'A' as u8 as usize
    } else {
        panic!("wtf")
    }
}

fn main() {
    let n: usize = parse_line().unwrap();
    let mut aa: Vec<usize> = parse_line().unwrap();
    let q: usize = parse_line().unwrap();
    let mut bb: Vec<usize> = vec![];
    for _ in 0..q {
        bb.push(parse_line().unwrap());
    }
    aa.sort();

    for b in bb {
        let bi = aa.lower_bound(b);
        let mut ans = 0;
        if bi == n {
            ans = (aa[n - 1] as isize - b as isize).abs();
        } else if bi == 0 {
            ans = (aa[bi] as isize - b as isize).abs();
        } else {
            ans += min(
                (aa[bi] as isize - b as isize).abs(),
                (aa[bi - 1] as isize - b as isize).abs(),
            );
        }
        println!("{}", ans);
    }
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
