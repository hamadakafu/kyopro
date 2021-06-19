use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();
    let mut uf = UnionFind::new(2 * 100000 + 1);

    for i in 0..aa.len() / 2 {
        if aa[i] != aa[n - 1 - i] {
            uf.union(aa[i], aa[n - 1 - i]);
        }
    }

    let mut hashmap = HashMap::new();
    for i in 0..=2 * 100000 {
        let e = hashmap.entry(uf.find(i)).or_insert(0);
        *e += 1;
    }

    let mut ans = 0;
    for (_, i) in hashmap {
        ans += i - 1;
    }
    println!("{}", ans);
}
#[derive(Debug)]
/// UnionFindは何らかの構造に対して同値類を求めるようなアルゴリズム
pub struct UnionFind {
    /// parents[index]はindexの対する親のindexが入っている
    pub parents: Vec<usize>,
    /// rankは子どもたちの数みたいなもの
    /// 大きい木に小さい木をつけることで効率よくfindできる
    pub rank: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parents: Vec<usize> = Vec::with_capacity(n);
        for i in 0..n {
            parents.push(i);
        }
        let rank = vec![1; n];
        UnionFind { parents, rank }
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
        if self.rank[x_root] > self.rank[y_root] {
            // x <--- y  y親がxになる
            self.parents[y_root] = x_root;
        } else {
            // y <--- x xの親がyになる
            self.parents[x_root] = y_root;

            // yの直下に1つ新しい子供としてxが増えるので
            // すでにxとyのランクが同じならば，yのrankを1増やす
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[y_root] += 1;
            }
        }
        return true;
    }
}
