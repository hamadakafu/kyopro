use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();

    // 有向木
    // a -> b: tree[a].push(b)
    let mut tree: Vec<HashSet<usize>> = vec![HashSet::new(); n + 1];
    for _ in 0..m {
        let k: usize = parse_line().unwrap();
        let aa: Vec<usize> = parse_line().unwrap();
        for i in 0..aa.len() - 1 {
            tree[aa[i]].insert(aa[i + 1]);
        }
    }

    let ans = topo_sort(n, tree);

    if ans.len() == n {
        // dbg!(tree, jisuu, ans);
        println!("Yes");
    } else {
        // dbg!(tree, jisuu, ans);
        println!("No");
    }
}

/// 有向グラフのトポロジカルソート
/// indexは1始まり
/// 0..=n
fn topo_sort(n: usize, paths: Vec<HashSet<usize>>) -> Vec<usize>{
    let mut ans = vec![];
    let mut jisuu: Vec<usize> = vec![0; n + 1];
    for t in paths.iter() {
        for i in t.iter() {
            jisuu[*i] += 1;
        }
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    for i in 1..=n {
        if jisuu[i] == 0 {
            queue.push_front(i);
            ans.push(i);
        }
    }

    // この段階で1は調査済みであるので，
    // 1に対して処理すべきことは済ましておく
    while !queue.is_empty() {
        let t = queue.pop_front().unwrap();
        for p in paths[t].iter() {
            // dbg!(p);
            jisuu[*p] -= 1;
            if jisuu[*p] == 0 {
                queue.push_front(*p);
                ans.push(*p);
            }
        }
    }
    ans
}
