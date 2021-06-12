use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

#[derive(Debug)]
struct Node {
    rank: u64,
    parent: u64,
    parents: HashSet<u64>,
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
        while left != right {
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
        while left != right {
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

fn main() {
    let n: usize = parse_line().unwrap();
    let pp: Vec<u64> = parse_line().unwrap();
    let mut children: Vec<Vec<u64>> = vec![vec![]; n + 1];

    for (i, &p) in pp.iter().enumerate() {
        children[p as usize].push(i as u64 + 2);
    }

    let mut treebyrank: Vec<Vec<u64>> = vec![vec![]; n];
    let mut ins = vec![0; n + 1];
    let mut outs = vec![0; n + 1];
    dfs(
        0,
        &mut vec![1],
        &children,
        &mut treebyrank,
        &mut ins,
        &mut outs,
        &mut 0,
    );

    // dbg!(&ins, &outs, &treebyrank);
    let q: usize = parse_line().unwrap();
    for _ in 0..q {
        let (u, d): (u64, u64) = parse_line().unwrap();
        let ans = {
            let a = vec![];
            let t = treebyrank.get(d as usize).unwrap_or(&a);
            t.lower_bound(outs[u as usize]) - t.lower_bound(ins[u as usize])
        };
        println!("{}", ans);
    }
}

fn dfs(
    depth: u64,
    queue: &mut Vec<u64>,
    children: &Vec<Vec<u64>>,
    tree_by_rank: &mut Vec<Vec<u64>>,
    ins: &mut Vec<u64>,
    outs: &mut Vec<u64>,
    counter: &mut u64,
) {
    if let Some(target) = queue.pop() {
        *counter += 1;
        ins[target as usize] = *counter;
        tree_by_rank[depth as usize].push(*counter);
        for c in children[target as usize].iter() {
            queue.push(*c);
            dfs(depth + 1, queue, children, tree_by_rank, ins, outs, counter);
        }
        *counter += 1;
        outs[target as usize] = *counter;
    } else {
        return;
    }
}
