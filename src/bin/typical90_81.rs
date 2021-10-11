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
    let (n, k): (usize, usize) = parse_line().unwrap();
    let ookisa = 5001;
    let mut heimen: Vec<Vec<isize>> = vec![vec![0; ookisa]; ookisa];
    for _ in 0..n {
        let (a, b): (usize, usize) = parse_line().unwrap();
        heimen[a][b] += 1;
    }

    // x方向に走査
    for x in 1..ookisa {
        for y in 0..ookisa {
            heimen[x][y] += heimen[x - 1][y];
        }
    }
    // y方向に走査
    for x in 0..ookisa {
        for y in 1..ookisa {
            heimen[x][y] += heimen[x][y - 1];
        }
    }

    // for i in 0..10 {
    //     for j in 0..10 {
    //         print!("{} ", heimen[i][j]);
    //     }
    //     println!();
    // }

    let mut ans = 0;
    for x in 0..ookisa - k - 1 {
        for y in 0..ookisa - k - 1 {
            ans = ans.max(
                heimen[x + k + 1][y + k + 1] + heimen[x][y]
                    - heimen[x + k + 1][y]
                    - heimen[x][y + k + 1],
            );
        }
    }
    println!("{}", ans);
}
