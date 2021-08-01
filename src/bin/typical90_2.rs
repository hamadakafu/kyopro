use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    if n % 2 == 1 {
        return;
    }
    dfs("".to_string(), n / 2, n / 2);
}

fn dfs(now: String, leftleft: usize, rightleft: usize) {
    if leftleft == 0 && rightleft == 0 {
        if check(now.clone()) {
            println!("{}", now);
        }
    } else if leftleft != 0 && rightleft != 0 {
        dfs(now.clone() + "(", leftleft - 1, rightleft);
        dfs(now.clone() + ")", leftleft, rightleft - 1);
    } else if rightleft != 0 {
        dfs(now.clone() + ")", leftleft, rightleft - 1);
    } else if leftleft != 0 {
        dfs(now.clone() + "(", leftleft - 1, rightleft);
    }
}

fn check(s: String) -> bool {
    let mut left = 0;
    let mut right = 0;
    for c in s.chars() {
        if c == '(' {
            left += 1;
        } else if c == ')' {
            right += 1;
        }
        if left < right {
            return false;
        }
    }
    true
}
