use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let aa: Vec<u64> = parse_line().unwrap();
    let bb: Vec<u64> = parse_line().unwrap();
    let cc: Vec<u64> = parse_line().unwrap();

    // num: count
    let mut map: HashMap<u64, u64> = HashMap::new();
    for c in cc.iter() {
        let e = map.entry(bb[*c as usize - 1] -1).or_insert(0);
        *e += 1;
    }

    let mut count = 0;
    for a in aa.iter() {
        count += map.get(&(a - &1)).unwrap_or(&0);
    }
    println!("{}", count);
}
