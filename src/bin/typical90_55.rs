use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (n, p, q): (usize, usize, usize) = parse_line().unwrap();
    let aa: Vec<usize> = parse_line().unwrap();

    let mut ans = 0;
    for a in aa.into_iter().combinations(5) {
        let tmp = (((((a[0] * a[1]) % p) * a[2]) % p) * ((a[3] * a[4]) % p)) % p;
        if tmp == q {
            ans += 1;
        }
    }
    println!("{}", ans);
}
