use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

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

// Union Find
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

// union findで扱えるようにindexの変換 (Z * Z) -> Z
fn rc2idx(r: usize, c: usize, rowsize: usize) -> usize {
    r * rowsize + c
}

fn main() {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let q: usize = parse_line().unwrap();
    let mut map: Vec<Vec<bool>> = vec![vec![false; w + 1]; h + 1];
    let mut uf = UnionFind::new((h + 1) * (w + 1));
    for _ in 0..q {
        let query: Vec<usize> = parse_line().unwrap();
        if query[0] == 1 {
            map[query[1]][query[2]] = true;
            if query[1] > 1 && map[query[1] - 1][query[2]] {
                let tmp = rc2idx(query[1], query[2], w);
                let tmp1 = rc2idx(query[1] - 1, query[2], w);
                uf.union(tmp, tmp1);
            }
            if query[2] > 1 && map[query[1]][query[2] - 1] {
                let tmp = rc2idx(query[1], query[2], w);
                let tmp1 = rc2idx(query[1], query[2] - 1, w);
                uf.union(tmp, tmp1);
            }
            if query[1] < h && map[query[1] + 1][query[2]] {
                let tmp = rc2idx(query[1], query[2], w);
                let tmp1 = rc2idx(query[1] + 1, query[2], w);
                uf.union(tmp, tmp1);
            }
            if query[2] < w && map[query[1]][query[2] + 1] {
                let tmp = rc2idx(query[1], query[2], w);
                let tmp1 = rc2idx(query[1], query[2] + 1, w);
                uf.union(tmp, tmp1);
            }
        } else {
            let (ra, ca, rb, cb) = (query[1], query[2], query[3], query[4]);
            let tmpa = rc2idx(ra, ca, w);
            let tmpb = rc2idx(rb, cb, w);
            if map[ra][ca] && map[rb][cb] && uf.find(tmpa) == uf.find(tmpb) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
