use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet};

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

// Euler tour
// オイラーツアーというらしい

fn main() {
    let n: usize = parse_line().unwrap();
    let mut paths: Vec<BinaryHeap<Reverse<usize>>> = vec![BinaryHeap::new(); n + 1];
    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(Reverse(b));
        paths[b].push(Reverse(a));
    }

    let mut pre = vec![0; n + 1];
    let mut already = vec![false; n + 1];
    already[1] = true;
    let mut michisuji = vec![1];

    let mut now = 1;
    print!("1");
    loop {
        if let Some(next) = paths[now].pop() {
            if !already[next.0] {
                pre[next.0] = now;
                now = next.0;
                already[now] = true;
                michisuji.push(now);
                continue;
            }
        } else {
            if now == 1 {
                break;
            } else {
                now = pre[now];
                michisuji.push(now);
            }
        }
    }

    for i in michisuji.iter().skip(1) {
        print!(" {}", i);
    }
    println!("");
}
