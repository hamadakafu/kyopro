use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let mut mods200: Vec<Vec<u64>> = vec![vec![]; 200];

    let n: u64 = whiteread::parse_line().unwrap();
    let aa: Vec<u64> = whiteread::parse_line().unwrap();

    for a in aa {
        mods200[(a % 200) as usize].push(a);
    }

    let mut count = 0;
    for m in mods200 {
        let n = m.len() as u128;
        if n < 2 {
            continue;
        }

        count += (n * (n - 1)) / 2;
    }
    println!("{}", count);
}
