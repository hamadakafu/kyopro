use itertools::Itertools;
use whiteread::parse_line;

use std::collections::HashSet;

fn main() {
    let n: u64 = parse_line().unwrap();
    let s: String = parse_line().unwrap();

    let mut one = HashSet::new();
    let mut two: HashSet<(usize, usize)> = HashSet::new();
    let mut three: HashSet<(usize, usize, usize)> = HashSet::new();
    // a b c
    for c in s.chars() {
        let ci = c.to_string().parse::<usize>().unwrap();
        for (o, t) in two.iter() {
            three.insert((*o, *t, ci));
        }
        for o in one.iter() {
            two.insert((*o, ci));
        }
        one.insert(ci);
    }
    println!("{}", three.len());
}
