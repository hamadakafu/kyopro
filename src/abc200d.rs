use whiteread::*;

fn main() {
    let n: u64 = parse_line().unwrap();
    let aa: Vec<u64> = parse_line().unwrap();
    let aa: Vec<u64> = aa
        .into_iter()
        .take(std::cmp::min(n, 8) as usize)
        .collect::<Vec<u64>>();

    let mut mods200: Vec<Vec<u64>> = vec![vec![]; 200];

    for mut i in 1..2_u64.pow(aa.len() as u32) {
        let mut bb = Vec::new();
        let ii = i;
        for a in aa.iter() {
            if i % 2 == 1 {
                bb.push(a);
            }
            i /= 2;
        }
        let bbsum: u64 = bb.into_iter().sum();
        mods200[(bbsum % 200) as usize].push(ii);
    }

    for m in mods200.into_iter() {
        if m.len() < 2 {
            continue;
        }
        let mut b = m[0];
        let mut c = m[1];
        println!("Yes");
        let mut bb = vec![];
        let mut cc = vec![];

        for i in 0..aa.len() {
            if b % 2 == 1 {
                bb.push(i + 1);
            }
            b /= 2;
            if c % 2 == 1 {
                cc.push(i + 1);
            }
            c /= 2;
        }
        print!("{}", bb.len());
        for bbb in bb.iter() {
            print!(" {}", bbb);
        }
        println!("");
        print!("{}", cc.len());
        for ccc in cc.iter() {
            print!(" {}", ccc);
        }
        println!("");

        return;
    }
    println!("No");
}
