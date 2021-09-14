use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let mut map: BTreeMap<&str, i32> = ["Alice", "Bob", "Carol", "Cheryl", "ccc"]
        .iter()
        .map(|&s| (s, 0))
        .collect();
    for (_, balance) in map.range_mut("B".."D") {
        *balance += 1;
    }
    for (_, balance) in map.range_mut("b".."d") {
        *balance += 10;
    }
    for (a, balance) in map.range_mut("B".."d") {
        *balance += 100;
        dbg!(a);
    }
    for (name, balance) in &map {
        println!("{} => {}", name, balance);
    }

    // これでは複数個に対応できないので
    let mut set: BTreeSet<usize> = vec![1, 1, 1, 1, 1, 2, 2].into_iter().collect();
    dbg!(set.range(0..2).count());
    // BTreeMapを使って個数を数える
    let tmp = vec![1, 1, 1, 1, 1, 2, 2];
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for i in tmp {
        let e = map.entry(i).or_default();
        *e += 1;
    }
    dbg!(map.range(0..2), map.range(0..4));

    let s = [0, 1, 1, 1, 1, 2, 2, 3, 5, 8, 13, 21, 34, 55];
    dbg!(s.binary_search(&1), s.binary_search(&2));
}
