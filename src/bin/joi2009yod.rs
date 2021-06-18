use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let m: usize = parse_line().unwrap();
    let n: usize = parse_line().unwrap();
    let mut map = vec![];
    for _ in 0..n {
        let line = parse_line().unwrap();
        map.push(line);
    }
    let mut ans = 0;
    for tate in 0..n {
        for yoko in 0..m {
            let tmp = dfs(n, m, (tate, yoko), &map, vec![vec![false; m]; n]);
            ans = max(ans, tmp);
        }
    }

    println!("{}", ans);
}

fn dfs(
    n: usize,
    m: usize,
    now: (usize, usize),
    map: &[Vec<usize>],
    mut arrived_map: Vec<Vec<bool>>,
) -> usize {
    if map[now.0][now.1] == 0 {
        return 0;
    }

    arrived_map[now.0][now.1] = true;
    let w = if now.1 > 0 && !arrived_map[now.0][now.1 - 1] {
        dfs(n, m, (now.0, now.1 - 1), &map, arrived_map.clone())
    } else {
        0
    };
    let e = if now.1 + 1 != m && !arrived_map[now.0][now.1 + 1] {
        dfs(n, m, (now.0, now.1 + 1), &map, arrived_map.clone())
    } else {
        0
    };
    let north = if now.0 > 0 && !arrived_map[now.0 - 1][now.1] {
        dfs(n, m, (now.0 - 1, now.1), &map, arrived_map.clone())
    } else {
        0
    };
    let s = if now.0 + 1 != n && !arrived_map[now.0 + 1][now.1] {
        dfs(n, m, (now.0 + 1, now.1), &map, arrived_map.clone())
    } else {
        0
    };

    return *vec![w, e, north, s].iter().max().unwrap() + 1;
}
