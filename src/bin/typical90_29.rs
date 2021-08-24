use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Add;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (w, n): (usize, usize) = parse_line().unwrap();

    let mut st: SegmentTree<Reverse<usize>> = SegmentTree::new(w, Reverse(0));
    for _ in 0..n {
        let (l, r): (usize, usize) = parse_line().unwrap();
        let now = st.query((l, r + 1));
        st.update((l, r + 1), Reverse(now.0 + 1));
        println!("{}", st.query((l, r + 1)).0);
    }
}

/// 遅延セグメントツリー
/// 2つの完全2分木のデータ構造
/// 区間の最小の値を保存する
struct SegmentTree<T> {
    // 区間の要素数
    range: usize,
    // 木の要素数: range * 2
    n: usize,
    // originalの木
    data: Vec<T>,
    // 遅延用の木
    lazy: Vec<T>,
    /// inf: 最小値を保存するときはstd::usize::MAX, 最大値を保存するときはReverse(0)
    INF: T,
}

impl<T: Ord + Clone + Debug> SegmentTree<T> {
    fn new(range: usize, INF: T) -> Self {
        let range = range.next_power_of_two();
        SegmentTree {
            range,
            n: range * 2 - 1,
            data: vec![INF.clone(); range * 2 - 1],
            lazy: vec![INF.clone(); range * 2 - 1],
            INF,
        }
    }

    /// 半開区間[a, b)の最小値(もしくは最大値)を求める
    fn query(&mut self, (a, b): (usize, usize)) -> T {
        self.query_sub((a, b), 0, (0, self.range))
    }

    fn query_sub(&mut self, (a, b): (usize, usize), i: usize, (l, r): (usize, usize)) -> T {
        self.eval(i);
        if r <= a || b <= l {
            // 完全に区間外
            self.INF.clone()
        } else if a <= l && r <= b {
            // 完全に区間内
            self.data[i].clone()
        } else {
            // 中途半端に区間が入っているとき
            let (cl, cr) = child(i);
            let vl = self.query_sub((a, b), cl, (l, (l + r) / 2));
            let vr = self.query_sub((a, b), cr, ((l + r) / 2, r));
            min(vl, vr)
        }
    }

    /// 半開区間[a, b)を指定してvalueで更新する(遅延)
    fn update(&mut self, (a, b): (usize, usize), value: T) {
        self.update_sub((a, b), value, 0, (0, self.range));
    }

    fn update_sub(&mut self, (a, b): (usize, usize), value: T, i: usize, (l, r): (usize, usize)) {
        // dbg!(a, b, &value, i, l, r);
        self.eval(i);
        if r <= a || b <= l {
            // 完全に区間外
            // do nothing
        } else if a <= l && r <= b {
            // 完全に区間内
            self.lazy[i] = value;
            self.eval(i);
        } else {
            // 中途半端に区間が入っているとき
            let (cl, cr) = child(i);
            self.update_sub((a, b), value.clone(), cl, (l, (l + r) / 2)); // 左の子
            self.update_sub((a, b), value.clone(), cr, ((l + r) / 2, r)); // 右の子
            self.data[i] = min(self.data[cl].clone(), self.data[cr].clone());
        }
    }

    fn eval(&mut self, i: usize) {
        if self.lazy[i] == self.INF {
            return;
        }
        // 葉じゃないなら子に伝搬
        if i < self.range - 1 {
            let (l, r) = child(i);
            self.lazy[l] = self.lazy[i].clone();
            self.lazy[r] = self.lazy[i].clone();
        }

        // originalの木に反映
        self.data[i] = self.lazy[i].clone();
        self.lazy[i] = self.INF.clone();
    }
}

/// rootが0始まりの場合のaのchild(left, right)を返す
fn child(a: usize) -> (usize, usize) {
    (a * 2 + 1, a * 2 + 2)
}

/// rootが0始まりの場合のaのparentを返す
fn parent(a: usize) -> usize {
    if a == 0 {
        panic!("0's root is invalid!");
    }
    (a - 1) / 2
}
