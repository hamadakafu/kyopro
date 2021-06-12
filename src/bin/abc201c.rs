use std::cmp::{max, min};
use std::collections::HashSet;

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let s: String = parse_line().unwrap();
    let oo = s
        .char_indices()
        .filter(|(_, c)| c == &'o')
        .map(|(i, _)| i as u64)
        .collect_vec();
    let xx = s
        .char_indices()
        .filter(|(_, c)| c == &'x')
        .map(|(i, _)| i as u64)
        .collect_vec();
    let hh = s
        .char_indices()
        .filter(|(_, c)| c == &'?')
        .map(|(i, _)| i as u64)
        .collect_vec();

    if oo.len() > 4 {
        println!("0");
        return;
    }
    let c = 4 - oo.len();
    let mut options = HashSet::new();
    let mut hhoo = hh.clone();
    hhoo.extend(oo.clone().into_iter());
    for mut bits in 0..(hh.len() + oo.len()).pow(c as u32) {
        let mut additional = vec![];
        for _ in 0..c {
            additional.push(hhoo[bits % (hh.len() + oo.len())]);
            bits /= hh.len() + oo.len();
        }
        let mut narabi = oo.clone();

        narabi.extend(additional.iter());
        for option in narabi.into_iter().permutations(4) {
            options.insert(option);
        }
    }
    println!("{}", options.len());
}

fn factorial(n: u64) -> u64 {
    let mut ans = 1;
    for i in 1..=n {
        ans *= n;
    }
    return ans;
}
