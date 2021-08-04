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

fn main() {
    let n: usize = parse_line().unwrap();
    let mut aaa: Vec<Vec<usize>> = vec![];
    aaa.push(vec![]);
    for _ in 0..n {
        aaa.push(parse_line().unwrap());
    }
    let m: usize = parse_line().unwrap();
    let mut kankei: Vec<Vec<bool>> = vec![vec![true; n + 1]; n + 1];
    for _ in 0..m {
        let (x, y): (usize, usize) = parse_line().unwrap();
        kankei[x][y] = false;
        kankei[y][x] = false;
    }

    let mut hm = HashSet::new();
    for i in 1..n + 1 {
        hm.insert(i);
    }
    let mut ans = std::usize::MAX;
    for last in 1..n + 1 {
        let mut tmp = hm.clone();
        tmp.remove(&last);
        let a = dfs(n, n - 1, last, tmp, &aaa, &kankei);
        if a == std::usize::MAX {
            continue;
        }
        ans = min(ans, a + aaa[last][0]);
    }
    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn dfs(
    N: usize,
    n: usize,
    last: usize,
    nokori: HashSet<usize>,
    aaa: &Vec<Vec<usize>>,
    kankei: &Vec<Vec<bool>>,
) -> usize {
    if nokori.is_empty() {
        return 0;
    }

    let mut ans = std::usize::MAX;
    for next in nokori.iter() {
        if !kankei[last][*next] {
            continue;
        }
        let mut nn = nokori.clone();
        nn.remove(next);
        let tmp = dfs(N, n - 1, *next, nn, aaa, kankei);
        if tmp == std::usize::MAX {
            continue;
        } else {
            ans = min(ans, tmp + aaa[*next][N - n]);
        }
    }

    ans
}
