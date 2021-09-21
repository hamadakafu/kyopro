#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let n: usize = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();

    let left = lis(&aa);
    let right = lis(&aa.clone().into_iter().rev().collect());
    // dbg!(&left, &right);
    let mut ans = std::usize::MIN;
    for i in 0..aa.len() {
        ans = ans.max(left[i] + right[n - i - 1] - 1);
    }
    println!("{}", ans);
}

/// 最長増加部分列
/// ans[i]はi番目の項を使った時のLIS
#[inline(always)]
fn lis(v: &Vec<usize>) -> Vec<usize> {
    assert_ne!(v.len(), 0);
    let mut b: Vec<usize> = vec![];
    let mut iis = vec![];
    for &i in v.iter() {
        let mut ii = b.lower_bound(i);
        // 同じのが来たときは donothing
        if ii >= b.len() {
            b.push(i);
        } else if ii < b.len() && b[ii] != i{
            b[ii] = i;
        }
        iis.push(ii + 1);
    }
    iis
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lis() {
        let case = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let ans = vec![1, 1, 2, 1, 3, 4, 2, 4];
        assert_eq!(ans, lis(&case));
    }
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
