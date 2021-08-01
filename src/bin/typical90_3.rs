use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
        paths[b].push(a);
    }
    let mut stack = vec![(1, 0)];
    let mut already = vec![false; n + 1];
    already[1] = true;
    let mut maxmachi = 1;
    let mut maxdis = 0;
    while !stack.is_empty() {
        let (machi, dis) = stack.pop().unwrap();
        for p in &paths[machi] {
            if already[*p] {
                continue;
            }
            stack.push((*p, dis + 1));
            if maxdis < dis + 1 {
                maxmachi = *p;
                maxdis = dis + 1;
            }
            already[*p] = true;
        }
    }

    let mut stack = vec![(maxmachi, 0)];
    let mut already = vec![false; n + 1];
    already[maxmachi] = true;
    let mut maxdis = 0;
    while !stack.is_empty() {
        let (machi, dis) = stack.pop().unwrap();
        for p in &paths[machi] {
            if already[*p] {
                continue;
            }
            stack.push((*p, dis + 1));
            if maxdis < dis + 1 {
                maxdis = dis + 1;
            }
            already[*p] = true;
        }
    }
    println!("{}", maxdis + 1);
}
