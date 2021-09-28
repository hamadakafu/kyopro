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
    let q: usize = parse_line().unwrap();
    let mut uf = UnionFind::new(n + 1);
    //omomis[i] == i + 1とi+2の重み
    let mut omomis: Vec<isize> = vec![0; n - 1];
    let mut ans: Vec<Option<(usize, usize, usize, isize)>> = vec![];
    for i in 0..q {
        let (t, x, y, v): (usize, usize, usize, isize) = parse_line().unwrap();
        if t == 0 {
            uf.union(x, y);
            omomis[x - 1] = v;
        } else {
            if uf.find(x) != uf.find(y) {
                ans.push(None);
            } else {
                ans.push(Some((t, x, y, v)));
            }
        }
    }
    let mut renketus: Vec<isize> = vec![0; n + 1];
    let mut renketus2 = vec![1; n + 1];
    let mut starti = 1;
    let mut i = 1;
    loop {
        if i >= n {
            break;
        }
        if uf.find(starti) == uf.find(i + 1) {
            let base = omomis[i - 1] - renketus[i];
            let base2 = omomis[i - 1] - renketus2[i];
            renketus[i + 1] = base;
            renketus2[i + 1] = base2;
            i += 1;
        } else {
            renketus[i + 1] = 0;
            renketus2[i + 1] = 1;
            i += 1;
            starti = i;
        }
    }

    let diff = renketus
        .iter()
        .zip(renketus2.iter())
        .map(|(a, b)| b - a)
        .collect_vec();
    // println!("{:?}", renketus);
    // println!("{:?}", omomis);
    // println!("{:?}", &diff);
    for a in ans.into_iter() {
        if let Some((t, x, y, v)) = a {
            println!("{}", renketus[y] - diff[x] * diff[y] * (renketus[x] - v));
        } else {
            println!("Ambiguous");
        }
    }
}
#[derive(Debug)]
/// UnionFindは何らかの構造に対して同値類を求めるようなアルゴリズム
pub struct UnionFind {
    /// parents[index]はindexの対する親のindexが入っている
    pub parents: Vec<usize>,
    /// sizes[index]は子どもたちの数みたいなもの
    /// 大きい木に小さい木をつけることで効率よくfindできる
    pub sizes: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parents: Vec<usize> = Vec::with_capacity(n);
        for i in 0..n {
            parents.push(i);
        }
        let sizes = vec![1; n];
        UnionFind { parents, sizes }
    }
    /// 再帰的にparentsからxの親を順番ずつ見ていき親を更新しつつxの親を求める
    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            let par = self.parents[x];
            let root = self.find(par);
            self.parents[x] = root;
            root
        }
    }
    /// union x and y
    /// ルートを付け替えるだけで，
    /// 新たに付け加えられた木の子どもたちの親は変わらないことに注意
    /// unionしたあとも find を呼ぶ必要がある．
    /// rootが同じ場合はunionに失敗する
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);
        // if x_root == y_root, do nothing
        if x_root == y_root {
            return false;
        }
        if self.sizes[x_root] > self.sizes[y_root] {
            // x <--- y  y親がxになる
            self.sizes[x_root] += self.sizes[y_root];
            self.parents[y_root] = x_root;
        } else {
            // y <--- x xの親がyになる
            self.sizes[y_root] += self.sizes[x_root];
            self.parents[x_root] = y_root;
        }
        true
    }

    /// 呼び出す前にfindを計算する必要はない
    pub fn size(&mut self, x: usize) -> usize {
        let r = self.find(x);
        self.sizes[r]
    }
}
