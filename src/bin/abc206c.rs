use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();
    let mut hashmap = HashMap::new();
    for a in aa.iter() {
        let e = hashmap.entry(*a).or_insert(0);
        *e += 1;
    }

    let mut values = hashmap.iter().collect_vec();
    let mut sums = VecDeque::new();

    sums.push_back(*values[values.len() - 1].1);
    for i in (1..values.len() - 1).rev() {
        sums.push_front(sums[0] + *values[i].1);
    }

    let mut ans = 0;
    for i in 0..values.len() - 1 {
        let tmp: usize = sums[i];
        ans += values[i].1 * tmp;
    }
    println!("{}", ans);
}
