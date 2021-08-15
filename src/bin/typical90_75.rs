use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let n: usize = parse_line().unwrap();
    let b = bunkai(n);
    dbg!(&b);
    if b.len() == 1{
        println!("0");
        return;
    }
    println!("{}", format!("{:b}", b.len() - 1).len());
}

fn bunkai(mut n: usize) -> Vec<usize> {
    let mut ans = vec![];
    let mut pivot = 2;

    while pivot * pivot <= n {
        while n / pivot * pivot == n {
            ans.push(pivot);
            n /= pivot;
        }
        pivot += 1;
    }
    if n != 1 {
        ans.push(n);
    }
    ans
}
