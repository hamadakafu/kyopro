use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut points: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        points.push(parse_line().unwrap());
    }

    let mut ans = 0.;
    let mut bunbo = 0.;
    for route in points.into_iter().permutations(n) {
        bunbo += 1.;
        let mut now = route[0];
        for p in route.into_iter().skip(1) {
            ans += (((now.0 - p.0).pow(2) + (now.1 - p.1).pow(2)) as f64).sqrt();
            now = p;
        }
    }
    println!("{}", ans / bunbo as f64);
}
