use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let mut aa: Vec<Vec<usize>> = vec![];
    for _ in 0..h {
        aa.push(parse_line().unwrap());
    }

    let mut gyousums: Vec<usize> = vec![];
    let mut retusums: Vec<usize> = vec![];

    for i in 0..h {
        gyousums.push(aa[i].iter().sum());
    }
    for i in 0..w {
        let mut tmp = 0;
        for j in 0..h {
            tmp += aa[j][i];
        }
        retusums.push(tmp);
    }

    for i in 0..h {
        for j in 0..w {
            if j == w - 1 {
                println!("{}", gyousums[i] + retusums[j] - aa[i][j]);
            } else {
                print!("{} ", gyousums[i] + retusums[j] - aa[i][j]);
            }
        }
    }
}
