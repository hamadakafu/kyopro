#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

/// 2の逆元 mod ten97．割りたいときに使う
const inv2ten97: u128 = 500_000_004;

// 01bfs
// 方向転換を1,そのまま進むを0コストにしてやる
fn main() {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let (rs, ss): (usize, usize) = parse_line().unwrap();
    let (rs, ss) = (rs - 1, ss - 1);
    let (rt, st): (usize, usize) = parse_line().unwrap();
    let (rt, st) = (rt - 1, st - 1);
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let line: String = parse_line().unwrap();
        map.push(line.chars().collect_vec());
    }

    let mut dist = vec![vec![vec![std::usize::MAX; 4]; w]; h];
    let mut already = vec![vec![vec![false; 4]; w]; h];
    let mut queue = VecDeque::new();
    for i in 0..4 {
        already[rs][ss][i] = true;
        queue.push_back((rs, ss, i));
        dist[rs][ss][i] = 0;
    }
    // この段階で1は調査済みであるので，
    // 1に対して処理すべきことは済ましておく
    while !queue.is_empty() {
        let (r, c, nowd) = queue.pop_front().unwrap();

        // dbg!(r, c, nowd, dist[r][c][nowd]);
        if r == rt && c == st {
            println!("{}", dist[r][c][nowd]);
            return;
        }
        for nextd in 0..4 {
            if nowd == nextd {
                // dbg!(
                //     nextd == 1
                //         && c < w - 1
                //         && map[r][c + 1] == '.'
                //         && !already.contains(&(r, c + 1, nextd))
                // );
                if nextd == 0 && r > 0 && map[r - 1][c] == '.' && !already[r - 1][c][nextd] {
                    already[r - 1][c][nextd] = true;
                    if dist[r - 1][c][nextd] > dist[r][c][nextd] {
                        queue.push_front((r - 1, c, nowd));
                        dist[r - 1][c][nextd] = dist[r][c][nextd];
                    }
                } else if nextd == 1
                    && c < w - 1
                    && map[r][c + 1] == '.'
                    && !already[r][c + 1][nextd]
                {
                    already[r][c + 1][nextd] = true;
                    if dist[r][c + 1][nextd] > dist[r][c][nextd] {
                        queue.push_front((r, c + 1, nowd));
                        dist[r][c + 1][nextd] = dist[r][c][nextd];
                    }
                } else if nextd == 2
                    && r < h - 1
                    && map[r + 1][c] == '.'
                    && !already[r + 1][c][nextd]
                {
                    already[r + 1][c][nextd] = true;
                    if dist[r + 1][c][nextd] > dist[r][c][nextd] {
                        dist[r + 1][c][nextd] = dist[r][c][nextd];
                        queue.push_front((r + 1, c, nowd));
                    }
                } else if nextd == 3 && c > 0 && map[r][c - 1] == '.' && !already[r][c - 1][nextd] {
                    already[r][c - 1][nextd] = true;
                    if dist[r][c - 1][nextd] > dist[r][c][nextd] {
                        dist[r][c - 1][nextd] = dist[r][c][nextd];
                        queue.push_front((r, c - 1, nowd));
                    }
                }
            } else {
                if !already[r][c][nextd] {
                    // push_backするのでpush_frontを遮らないようにフラグは建てない
                    if dist[r][c][nextd] > dist[r][c][nowd] + 1 {
                        dist[r][c][nextd] = dist[r][c][nowd] + 1;
                        queue.push_back((r, c, nextd));
                    }
                }
            }
        }
    }
}
