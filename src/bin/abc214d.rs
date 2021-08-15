use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut edges = vec![];
    for _ in 0..n - 1 {
        let (u, v, w): (usize, usize, usize) = parse_line().unwrap();
        edges.push((w, u, v));
    }

    edges.sort();

    let mut uf = UnionFind::new(n + 1);
    let mut ans = 0;
    for (w, u, v) in edges {
        ans += w * uf.size(u) * uf.size(v);
        uf.union(u, v);
    }
    println!("{}", ans);
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
