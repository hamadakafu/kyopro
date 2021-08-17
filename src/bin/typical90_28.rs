use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000_000_007;

// いもす法
fn main() {
    let n: usize = parse_line().unwrap();
    let ookisa = 1002;
    let mut heimen: Vec<Vec<isize>> = vec![vec![0; ookisa]; ookisa];
    for _ in 0..n {
        let (lx, ly, rx, ry): (usize, usize, usize, usize) = parse_line().unwrap();
        heimen[lx][ly] += 1;
        heimen[rx][ry] += 1;
        heimen[lx][ry] -= 1;
        heimen[rx][ly] -= 1;
    }

    for x in 1..ookisa {
        for y in 0..ookisa {
            heimen[x][y] += heimen[x - 1][y];
        }
    }
    for x in 0..ookisa {
        for y in 1..ookisa {
            heimen[x][y] += heimen[x][y - 1];
        }
    }
    let mut ans: Vec<usize> = vec![0; n + 1];
    for x in 0..ookisa {
        for y in 0..ookisa {
            ans[heimen[x][y] as usize] += 1;
        }
    }

    // dbg!(&heimen, &step2_heimen, &step3_heimen);
    for a in ans.into_iter().skip(1) {
        println!("{}", a);
    }
}
