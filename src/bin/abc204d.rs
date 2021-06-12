use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let tt: Vec<usize> = parse_line().unwrap();
    let ttsum: usize = tt.iter().sum();

    let mut left = if ttsum % 2 == 1 {
        ttsum / 2 + 1
    } else {
        ttsum / 2
    };

    while !check(left, &tt) {
        left += 1;
    }
    println!("{}", left);
}

fn check(value: usize, tt: &[usize]) -> bool {
    let mut dp: Vec<bool> = vec![false; value + 1];
    dp[0] = true;

    for i in 0..tt.len() {
        if value < tt[i] {
            continue;
        }
        for j in (0..=value - tt[i]).rev() {
            if dp[j] {
                dp[j + tt[i]] = true;
            }
        }
    }
    //    dbg!(&dp);
    dp[value]
}

/// u32, u64 or usize
trait Bubunwa {
    fn bubunwa_usize(&self, value: usize) -> bool;
    fn bubunwa_u64(&self, value: u64) -> bool;
}

impl Bubunwa for Vec<usize> {
    fn bubunwa_usize(&self, value: usize) -> bool {
        let mut dp: Vec<bool> = vec![false; value + 1];
        dp[0] = true;

        for i in 0..self.len() {
            if value < self[i] {
                continue;
            }
            for j in (0..=value - self[i]).rev() {
                if dp[j] {
                    dp[j + self[i]] = true;
                }
            }
        }
        //    dbg!(&dp);
        dp[value]
    }

    fn bubunwa_u64(&self, value: u64) -> bool {
        let value: usize = value as usize;
        let mut dp: Vec<bool> = vec![false; value + 1];
        dp[0] = true;

        for i in 0..self.len() {
            if value < self[i] {
                continue;
            }
            for j in (0..=value - self[i]).rev() {
                if dp[j] {
                    dp[j + self[i]] = true;
                }
            }
        }
        //    dbg!(&dp);
        dp[value]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubunwa_usize() {
        let cases = vec![(
            vec![8, 3, 7, 2, 5],
            (0..20_usize),
            vec![
                true, false, true, true, false, true, false, true, true, true, true, true, true,
                true, true, true, true, true, true, false,
            ],
        )];
        for case in cases {
            for (value, &real) in case.1.zip(case.2.iter()) {
                assert_eq!(case.0.bubunwa_usize(value), real);
            }
        }
    }
    #[test]
    fn test_bubunwa_u64() {
        let cases = vec![(
            vec![8, 3, 7, 2, 5],
            (0..20_u64),
            vec![
                true, false, true, true, false, true, false, true, true, true, true, true, true,
                true, true, true, true, true, true, false,
            ],
        )];
        for case in cases {
            for (value, &real) in case.1.zip(case.2.iter()) {
                assert_eq!(case.0.bubunwa_u64(value), real);
            }
        }
    }
}
