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

// ダイクストラ法を2方向からやると両方向の最短距離が求まる
fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut paths: Vec<Vec<(usize, usize)>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let (a, b, c): (usize, usize, usize) = parse_line().unwrap();
        paths[a].push((b, c));
        paths[b].push((a, c));
    }

    let mut queue: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    let mut already: Vec<bool> = vec![false; n + 1];
    let mut dis1: Vec<usize> = vec![std::usize::MAX; n + 1];
    queue.push((Reverse(0), 1));
    already[1] = true;
    dis1[1] = 0;
    while !queue.is_empty() {
        let (Reverse(hun), machi) = queue.pop().unwrap();
        dis1[machi] = min(dis1[machi], hun);
        already[machi] = true;

        // // 最後の街なら
        // if machi == n {
        //     break;
        // }

        for p in paths[machi].iter() {
            if !already[p.0] {
                queue.push((Reverse(hun + p.1), p.0));
            }
        }
    }
    let mut queue: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    let mut already: Vec<bool> = vec![false; n + 1];
    let mut dis2: Vec<usize> = vec![std::usize::MAX; n + 1];
    queue.push((Reverse(0), n));
    already[n] = true;
    dis2[n] = 0;
    while !queue.is_empty() {
        let (Reverse(hun), machi) = queue.pop().unwrap();
        dis2[machi] = min(dis2[machi], hun);
        already[machi] = true;

        // // 最後の街なら
        // if machi == 0 {
        //     break;
        // }

        for p in paths[machi].iter() {
            if !already[p.0] {
                queue.push((Reverse(hun + p.1), p.0));
            }
        }
    }

    for k in 1..=n {
        println!("{}", dis1[k] + dis2[k]);
    }
}
