use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();
    // let aa = aa.into_iter().map(|a| a as u128).collect_vec();

    let mut primes: HashSet<usize> = HashSet::new();
    for a in aa.iter() {
        for b in bunkai(*a) {
            primes.insert(b);
        }
    }
    let mut ans = vec![1];

    for i in 2..=m {
        let tmp = bunkai(i);
        let mut flag = true;
        for j in tmp {
            if primes.contains(&j) {
                flag = false;
                break;
            }
        }
        if flag {
            ans.push(i);
        }
    }

    println!("{}", ans.len());
    for i in ans {
        println!("{}", i);
    }
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
fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

fn gcd(a: u128, b: u128) -> u128 {
    if a > b {
        let tmp = a % b;
        if tmp == 0 {
            b
        } else {
            gcd(b, tmp)
        }
    } else {
        let tmp = b % a;
        if tmp == 0 {
            a
        } else {
            gcd(a, tmp)
        }
    }
}
