use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (h, w, c): (usize, usize, usize) = parse_line().unwrap();
    let mut aa: Vec<Vec<usize>> = vec![];
    for _ in 0..h {
        aa.push(parse_line().unwrap());
    }

    let mut ans = std::usize::MAX;

    let mut migishita = vec![vec![std::usize::MAX; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                migishita[i][j] = aa[i][j];
            } else if i == 0 && j != 0 {
                migishita[i][j] = min(aa[i][j], migishita[i][j - 1] + c);
            } else if i != 0 && j == 0 {
                migishita[i][j] = min(aa[i][j], migishita[i - 1][j] + c);
            } else if i != 0 && j != 0 {
                migishita[i][j] = vec![aa[i][j], migishita[i][j - 1] + c, migishita[i - 1][j] + c]
                    .into_iter()
                    .min()
                    .unwrap();
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                continue;
            } else if i == 0 && j != 0 {
                ans = min(ans, aa[i][j] + migishita[i][j - 1] + c);
            } else if i != 0 && j == 0 {
                ans = min(ans, aa[i][j] + migishita[i - 1][j] + c);
            } else if i != 0 && j != 0 {
                ans = min(ans, aa[i][j] + migishita[i - 1][j] + c);
                ans = min(ans, aa[i][j] + migishita[i][j - 1] + c);
            }
        }
    }

    let mut migishita = vec![vec![std::usize::MAX; w]; h];
    for i in 0..h {
        for j in (0..w).rev() {
            if i == 0 && j == w - 1 {
                migishita[i][j] = aa[i][j];
            } else if i == 0 && j != w - 1 {
                migishita[i][j] = min(aa[i][j], migishita[i][j + 1] + c);
            } else if i != 0 && j == w - 1 {
                migishita[i][j] = min(aa[i][j], migishita[i - 1][j] + c);
            } else if i != 0 && j != w - 1 {
                migishita[i][j] = vec![aa[i][j], migishita[i][j + 1] + c, migishita[i - 1][j] + c]
                    .into_iter()
                    .min()
                    .unwrap();
            }
        }
    }
    for i in 0..h {
        for j in (0..w).rev() {
            if i == 0 && j == w - 1 {
                continue;
            } else if i == 0 && j != 0 {
                ans = min(ans, aa[i][j] + migishita[i][j + 1] + c);
            } else if i != 0 && j == 0 {
                ans = min(ans, aa[i][j] + migishita[i - 1][j] + c);
            } else if i != 0 && j != w - 1 {
                ans = min(ans, aa[i][j] + migishita[i - 1][j] + c);
                ans = min(ans, aa[i][j] + migishita[i][j + 1] + c);
            }
        }
    }

    println!("{}", ans);
}
