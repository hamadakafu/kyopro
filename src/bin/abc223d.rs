#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
    }
    let top = topo_sort(n, paths);
    if top.len() == n {
        for i in 0..top.len() - 1 {
            print!("{} ", top[i]);
        }
        println!("{}", top.last().unwrap());
    } else {
        // dbg!(top);
        println!("-1");
    }
}

/// 有向グラフのトポロジカルソート
/// indexは1始まりで頂点数nのグラフ
/// 有向グラフが巡回していなければトポロジカルソートソートで長さがnのVecが出てくる
/// そうでないならトポロジカルソートできていない（ノードが重複して出現してしまうので）
fn topo_sort(n: usize, paths: Vec<Vec<usize>>) -> Vec<usize> {
    let mut ans = vec![];
    // 次数は入ってくるパスの数
    let mut jisuu: Vec<usize> = vec![0; n + 1];
    for t in paths.iter() {
        for i in t.iter() {
            jisuu[*i] += 1;
        }
    }

    let mut queue: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut already = vec![false; n + 1];
    // 0が入らないように1始まりにしている
    for i in 1..=n {
        if jisuu[i] == 0 {
            queue.push(Reverse(i));
            already[i] = true;
        }
    }

    while !queue.is_empty() {
        let t = queue.pop().unwrap().0;
        ans.push(t);
        for p in paths[t].iter() {
            jisuu[*p] -= 1;
            if jisuu[*p] == 0 {
                queue.push(Reverse(*p));
            }
        }
    }
    ans
}
