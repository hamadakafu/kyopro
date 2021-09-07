use std::collections::BTreeMap;

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

}
