use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};
use std::collections::BinaryHeap;

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, k): (usize, u64) = parse_line().unwrap();
    let mut friends: Vec<(u64, u64)>= vec![];

    for _ in 0..n {
        friends.push(parse_line().unwrap());
    }
    friends.sort_by(|a, b| a.0.cmp(&b.0));

    let mut money = k;
    let mut nowmura = 0;
    let mut nextfid = 0;
    loop {
        let nowf = friends[nextfid];
        if nowf.0 - nowmura > money {
            nowmura += money;
            break
        }
        money -= nowf.0 - nowmura;
        money += nowf.1;
        nowmura = nowf.0;
        nextfid += 1;
        if nextfid == friends.len() {
            nowmura += money;
            break;
        }
    }
    println!("{}", nowmura);
}
