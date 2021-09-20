/// O(sqrt(N))
/// 与えられた数の素因数分解を行う
/// TODO: HashMap<usize, usize>にして素因数の数を数えても良いかも知れない
fn bunkai(mut n: usize) -> Vec<usize> {
    let mut ans = vec![];
    let mut pivot = 2;

    while pivot * pivot <= n {
        while n / pivot * pivot == n {
            ans.push(pivot);
            n /= pivot;
        }
        pivot += 1;
    }
    if n != 1 {
        ans.push(n);
    }
    ans
}

fn main() {

}
