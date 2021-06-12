// Atcoderの問題 https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_c

use whiteread::parse_line;
use itertools::Itertools;

fn main() {
    let nm: Vec<usize> = parse_line().unwrap();
    let n = nm[0];
    let m = nm[1];

    let mut aa: Vec<Vec<u64>> = vec![];

    for _ in 0..n {
        let aline: Vec<u64> = parse_line().unwrap();
        aa.push(aline);
    }

    let mut saikou = 0;
    for kyoku in (0..m).into_iter().combinations(2) {
        let mut tmp = 0;
        for i in 0..n {
            tmp += std::cmp::max(aa[i][kyoku[0]], aa[i][kyoku[1]]);
        }
        saikou = std::cmp::max(saikou, tmp);
    }

    println!("{}", saikou);
}
