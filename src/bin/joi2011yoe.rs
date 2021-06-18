use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::VecDeque;
use whiteread::parse_line;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let (h, w, n): (usize, usize, usize) = parse_line().unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let tmp: String = parse_line().unwrap();
        map.push(tmp.chars().collect_vec());
    }

    let mut ans = 0;
    let zahyo = find_s(n, &map);

    for i in 1..=n {
        let mut already = vec![vec![false; w]; h];
        let mut queue = VecDeque::new();
        let (sh, sw) = zahyo[i - 1];
        let (gh, gw) = zahyo[i];
        already[sh][sw] = true;
        queue.push_back((sh, sw, 0));
        ans += loop {
            let (nowh, noww, count) = queue.pop_front().unwrap();

            if gh == nowh && gw == noww {
                break count;
            }
            if noww > 0 && !already[nowh][noww - 1] && map[nowh][noww - 1] != 'X' {
                queue.push_back((nowh, noww - 1, count + 1));
                already[nowh][noww - 1] = true;
            }
            if noww + 1 != w && !already[nowh][noww + 1] && map[nowh][noww + 1] != 'X' {
                queue.push_back((nowh, noww + 1, count + 1));
                already[nowh][noww + 1] = true;
            }
            if nowh > 0 && !already[nowh - 1][noww] && map[nowh - 1][noww] != 'X' {
                queue.push_back((nowh - 1, noww, count + 1));
                already[nowh - 1][noww] = true;
            }
            if nowh + 1 != h && !already[nowh + 1][noww] && map[nowh + 1][noww] != 'X' {
                queue.push_back((nowh + 1, noww, count + 1));
                already[nowh + 1][noww] = true;
            }
        };
    }
    println!("{}", ans);
}

fn find_s(n: usize, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ans = vec![];
    for h in 0..map.len() {
        for w in 0..map[0].len() {
            if map[h][w] == 'S' {
                ans.push((h, w));
            }
        }
    }
    for i in 1..=n {
        for h in 0..map.len() {
            for w in 0..map[0].len() {
                if map[h][w] == std::char::from_digit(i as u32, 10).unwrap() {
                    ans.push((h, w));
                }
            }
        }
    }
    ans
}
