use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let s: String = parse_line().unwrap();
    let m: usize = 1000000007;

    let mut cc = 0;
    let mut hh = 0;
    let mut oo = 0;
    let mut kk = 0;
    let mut uu = 0;
    let mut dd = 0;
    let mut aa = 0;
    let mut ii = 0;

    for c in s.chars() {
        match c {
            'c' => {
                cc += 1;
                cc %= m;
            }
            'h' => {
                hh += cc;
                hh %= m;
            }
            'o' => {
                oo += hh;
                oo %= m;
            }
            'k' => {
                kk += oo;
                kk %= m;
            }
            'u' => {
                uu += kk;
                uu %= m;
            }
            'd' => {
                dd += uu;
                dd %= m;
            }
            'a' => {
                aa += dd;
                aa %= m;
            }
            'i' => {
                ii += aa;
                ii %= m;
            }
            _ => {}
        }
    }
    println!("{}", ii);
}
