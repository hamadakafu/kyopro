use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use whiteread::parse_line;

const ten97: usize = 1000000007;
fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let mut paths: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let (a, b): (usize, usize) = parse_line().unwrap();
        paths[a].push(b);
        paths[b].push(a);
    }

    let mut preranks: HashMap<usize, usize> = HashMap::new();
    let mut nextranks: HashMap<usize, usize> = HashMap::new();
    preranks.insert(1, 1);
    let mut already: HashSet<usize> = HashSet::new();
    already.insert(1);

    let mut rank = 0;
    loop {
        if rank == n {
            break;
        }
        for (machi, num) in preranks.iter() {
            if *machi == n {
                // dbg!(n, num, &preranks);
                println!("{}", num);
                return;
            }
            for p in paths[*machi].iter() {
                if already.contains(p) {
                    // dbg!(rank, machi, p);
                    continue;
                }
                let e = nextranks.entry(*p).or_insert(0);
                *e += preranks.get(machi).unwrap();
                *e %= ten97;
            }
        }
        for p in nextranks.keys() {
            already.insert(*p);
        }
        preranks = nextranks;
        nextranks = HashMap::new();
        rank += 1;
    }
    println!("0");
}
