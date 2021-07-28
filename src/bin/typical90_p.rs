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
    let mut coins: Vec<usize> = parse_line().unwrap();

    coins.sort_by(|a, b| b.cmp(&a));

    let mut ans = std::usize::MAX;
    for i in (0..=9999).rev() {
        for j in (0..=9999 - i).rev() {
            if n < coins[0] * i + coins[1] * j {
                continue;
            }
            let k = (n - coins[0] * i - coins[1] * j) / coins[2];
            if coins[0] * i + coins[1] * j + coins[2] * k == n {
                ans = min(ans, i + j + k);
            }
        }
    }
    println!("{}", ans);
}
