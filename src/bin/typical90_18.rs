use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;

fn main() {
    let t: f64 = parse_line().unwrap();
    let (l, x, y): (f64, f64, f64) = parse_line().unwrap();
    let q: usize = parse_line().unwrap();
    for _ in 0..q {
        let e: f64 = parse_line().unwrap();
        let (ky, kz) = yz(t, e, l);
        println!(
            "{}",
            (kz / ((y - ky).powi(2) + x.powi(2)).sqrt()).atan() * 360.0
                / 2.0
                / std::f64::consts::PI
        );
    }
}

fn yz(t: f64, mut e: f64, l: f64) -> (f64, f64) {
    let tmp = (e / t) as isize;
    e -= t * tmp as f64;
    let mut bun = e / t;
    bun -= 0.25;

    let (s, c) = (2. * std::f64::consts::PI * bun).sin_cos();
    (-c * l / 2., s * l / 2. + l / 2.)
}
