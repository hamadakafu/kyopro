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
    let (w, n): (usize, usize) = parse_line().unwrap();
    let mut items: Vec<(usize, usize, isize)> = vec![];
    for _ in 0..n {
        let item: (usize, usize, isize) = parse_line().unwrap();
        items.push(item);
    }
    let mut dp_pre: SegmentTree<Reverse<isize>> = SegmentTree::new(w + 1, Reverse(-1));
    let mut dp_next: SegmentTree<Reverse<isize>> = SegmentTree::new(w + 1, Reverse(-1));
    dp_pre.update((items[0].0, items[0].1 + 1), Reverse(items[0].2));
    dp_pre.update((0, 1), Reverse(0));
    for i in 2..=n {
        let nowitem = items[i - 1];
        for j in 0..=w {
            let l = max(0, j as isize - nowitem.1 as isize) as usize;
            let r = (max(-1, j as isize - nowitem.0 as isize) + 1) as usize;
            let tmp = dp_pre.query((l, r)).0;
            let mm = dp_pre.query((j, j + 1));
            // jと等しくなる香辛料が見当たらないときは前のを引き継ぐ
            if tmp == -1 {
                dp_next.update((j, j + 1), mm);
                continue;
            }
            let m = tmp + nowitem.2;
            // if i == 3 {
            //     dbg!(i, j, l, r, mm, m);
            // }
            dp_next.update((j, j + 1), Reverse(max(mm.0, m)));
        }
        dp_pre = dp_next;
        dp_next = SegmentTree::new(w + 1, Reverse(-1));
    }
    let ans = dp_pre.query((w, w + 1));
    println!("{}", ans.0);
}

/// 遅延セグメントツリー
/// 2つの完全2分木のデータ構造
/// 区間の最小(or 最大)の値を保存する
/// 最大値を保存するときはSegmentTree<Reverse<usize>>
#[derive(Debug, Clone)]
struct SegmentTree<T> {
    // 区間の要素数
    range: usize,
    // 木の要素数: range * 2
    n: usize,
    // originalの木
    data: Vec<T>,
    // 遅延用の木
    lazy: Vec<T>,
    /// inf: 最小値を保存するときはstd::usize::MAX, 最大値を保存するときはReverse(0) or
    /// Reverse(-1)とか
    INF: T,
}

impl<T: Ord + Clone + Debug> SegmentTree<T> {
    #[inline(always)]
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
    #[inline(always)]
    fn query(&mut self, (a, b): (usize, usize)) -> T {
        self.query_sub((a, b), 0, (0, self.range))
    }

    #[inline(always)]
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
    #[inline(always)]
    fn update(&mut self, (a, b): (usize, usize), value: T) {
        self.update_sub((a, b), value, 0, (0, self.range));
    }

    #[inline(always)]
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
            self.update_sub((a, b), value, cr, ((l + r) / 2, r)); // 右の子
            self.data[i] = min(self.data[cl].clone(), self.data[cr].clone());
        }
    }

    #[inline(always)]
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
#[inline(always)]
const fn child(a: usize) -> (usize, usize) {
    (a * 2 + 1, a * 2 + 2)
}

