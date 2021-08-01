use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

// 辞書準部分文字列は表を作るのがコツ
// 後ろから文字列を見て表を作る
fn main() {
    let (n, k): (usize, usize) = parse_line().unwrap();
    let s: String = parse_line().unwrap();

    let mut ss: Vec<(usize, char)> = s.chars().enumerate().collect_vec();
    if n == k {
        println!("{}", s);
        return;
    }

    let mut memo: Vec<Vec<isize>> = vec![vec![-1; n + 1]; 26];
    for (i, sc) in ss.iter().rev() {
        for c in (b'a'..=b'z').map(char::from) {
            if *sc == c {
                memo[alphabet2idx(c)][*i] = *i as isize;
            } else {
                memo[alphabet2idx(c)][*i] = memo[alphabet2idx(c)][*i + 1];
            }
        }
    }

    let mut currentidx = 0;
    let mut currentlen = 0;
    let mut ans = "".to_string();
    while currentlen != k {
        for c in (b'a'..=b'z').map(char::from) {
            let ci = alphabet2idx(c);
            if memo[ci][currentidx] != -1
                && n as isize - memo[ci][currentidx] >= k as isize - currentlen as isize
            {
                ans.push(c);
                currentidx = memo[ci][currentidx] as usize + 1;
                currentlen += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}

fn alphabet2idx(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as u8 as usize - 'a' as u8 as usize
    } else if c.is_ascii_uppercase() {
        c as u8 as usize - 'A' as u8 as usize
    } else {
        panic!("wtf")
    }
}
