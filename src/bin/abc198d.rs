use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use whiteread::parse_line;

fn main() {
    let s1: String = parse_line().unwrap();
    let s2: String = parse_line().unwrap();
    let s3: String = parse_line().unwrap();
    let mut d = HashSet::new();
    for c in s1.clone().chars() {
        d.insert(c);
    }
    for c in s2.clone().chars() {
        d.insert(c);
    }
    for c in s3.clone().chars() {
        d.insert(c);
    }
    if d.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }

    let cs = d.iter().collect_vec();

    let mut m = HashMap::new();
    for l in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        .iter()
        .permutations(d.len())
    {
        for (k, v) in cs.clone().into_iter().zip(l) {
            m.insert(k, *v);
        }
        let mut s1i = String::new();
        let mut s2i = String::new();
        let mut s3i = String::new();
        for c in s1.clone().chars() {
            s1i.push(m[&c]);
        }
        for c in s2.clone().chars() {
            s2i.push(m[&c]);
        }
        for c in s3.clone().chars() {
            s3i.push(m[&c]);
        }

        let s1ii = s1i.parse::<u64>().unwrap();
        let s2ii = s2i.parse::<u64>().unwrap();
        let s3ii = s3i.parse::<u64>().unwrap();
        if s1ii + s2ii == s3ii
            && s1.len() == format!("{}", s1ii).len()
            && s2.len() == format!("{}", s2ii).len()
            && s3.len() == format!("{}", s3ii).len()
            && s1ii > 0
            && s2ii > 0
            && s3ii > 0
        {
            println!("{} {} {}", s1ii, s2ii, s3ii);
            return;
        }
    }
    println!("UNSOLVABLE");
}
