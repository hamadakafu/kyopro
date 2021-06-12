fn main() {
    loop {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let nx = input
            .trim()
            .split(' ')
            .map(|i| {
                return i.parse::<u64>().unwrap();
            })
            .collect::<Vec<u64>>();
        if nx[0] == 0 && nx[1] == 0 {
            return;
        }
        println!("{}", solve(nx[0], nx[1]));
    }
}

fn combination<T: Clone>(xs: Vec<T>, n: u64) -> Vec<Vec<T>> {
    if (xs.len() as u64) < n {
        return vec![];
    }

    let mut ans = vec![];

    if n == 1 {
        return xs.into_iter().map(|x| vec![x]).collect();
    }
    let mut xs_1 = xs.clone();
    let x = xs_1.pop().unwrap();

    let xs_1 = combination(xs_1, n - 1);
    ans.append(
        &mut xs_1
            .into_iter()
            .map(|mut xs| {
                xs.push(x.clone());
                xs
            })
            .collect(),
    );
    let mut xs_1 = xs.clone();
    xs_1.pop();
    ans.append(&mut combination(xs_1, n));

    return ans;
}

fn solve(n: u64, x: u64) -> u64 {
    let vecs = combination((1..=n).collect(), 3);
    let mut count = 0;
    for v in vecs {
        if v.into_iter().sum::<u64>() == x {
            count += 1;
        }
    }
    return count;
}
