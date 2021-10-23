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
    let m: usize = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; 10];
    for _ in 0..m {
        let (u, v): (usize, usize) = parse_line().unwrap();
        paths[u].push(v);
        paths[v].push(u);
    }
    let pp: Vec<usize> = parse_line().unwrap();

    let mut already = HashMap::new();
    otozure(&mut already, &pp);
    if check(&pp) {
        println!("{}", 0);
        return;
    }
    let mut queue = VecDeque::new();
    nextpattern(&mut already, &paths, pp).into_iter().for_each(|p| {
        alreadyotozure(&already, &p);
        queue.push_back((1, p));
    });
    while !queue.is_empty() {
        let (dis, pattern) = queue.pop_front().unwrap();
        for p in nextpattern(&mut already, &paths, pattern) {
            if check(&p) {
                println!("{}", dis + 1);
                return;
            }
            queue.push_back((dis + 1, p));
        }
    }
    println!("{}", -1);
}

fn otozure(
    already: &mut HashMap<(usize, usize, usize, usize, usize, usize, usize, usize), bool>,
    pp: &Vec<usize>,
) {
    already.insert(
        (pp[0], pp[1], pp[2], pp[3], pp[4], pp[5], pp[6], pp[7]),
        true,
    );
}

fn alreadyotozure(
    already: &HashMap<(usize, usize, usize, usize, usize, usize, usize, usize), bool>,
    pp: &Vec<usize>,
) -> bool {
    *already
        .get(&(pp[0], pp[1], pp[2], pp[3], pp[4], pp[5], pp[6], pp[7]))
        .unwrap_or(&false)
}

fn check(now: &Vec<usize>) -> bool {
    now[0] == 1
        && now[1] == 2
        && now[2] == 3
        && now[3] == 4
        && now[4] == 5
        && now[5] == 6
        && now[6] == 7
        && now[7] == 8
}

fn nextpattern(
    already: &mut HashMap<(usize, usize, usize, usize, usize, usize, usize, usize), bool>,
    paths: &Vec<Vec<usize>>,
    now: Vec<usize>,
) -> Vec<Vec<usize>> {
    let mut ans = vec![];
    let empty = 45 - now[0] - now[1] - now[2] - now[3] - now[4] - now[5] - now[6] - now[7];

    for tonari in paths[empty].iter() {
        for (i, n) in now.iter().enumerate() {
            if n == tonari {
                let mut tmp = now.clone();
                tmp[i] = empty;
                if alreadyotozure(already, &tmp) {
                    continue;
                }
                otozure(already, &tmp);
                ans.push(tmp);
            }
        }
    }

    ans
}
