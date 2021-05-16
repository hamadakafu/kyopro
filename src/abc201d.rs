use std::cmp::{max, min};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let mut map = vec![];
    let mut memo: Vec<Vec<Option<(i64, i64)>>> = vec![vec![None; w]; h];
    for _ in 0..h {
        let line: Vec<char> = parse_line::<String>().unwrap().chars().collect_vec();
        map.push(line);
    }
    if h == 1 && w == 1 {
        println!("Draw");
        return;
    }
    saiteki(h, w, &map, 0, 0, &mut memo, true);
    // dbg!(&memo);
    let takahashi = memo[0][0].unwrap().0;
    let aoki = memo[0][0].unwrap().1;
    // let aoki: i64 = if h == 1 {
    //     memo[0][1].unwrap().1
    // } else if w == 1 {
    //     memo[1][0].unwrap().1
    // } else {
    //     max(memo[0][1].unwrap().1, memo[1][0].unwrap().1)
    // };
    if takahashi > aoki {
        println!("Takahashi");
    } else if takahashi == aoki {
        println!("Draw");
    } else {
        println!("Aoki");
    }
}

fn saiteki(
    h: usize,
    w: usize,
    map: &Vec<Vec<char>>,
    tate: usize,
    yoko: usize,
    // takahashi, aoki
    memo: &mut Vec<Vec<Option<(i64, i64)>>>,
    takahasi: bool,
) -> (i64, i64) {
    // ついた先には進めないなら
    if tate + 1 == h && yoko + 1 == w {
        memo[tate][yoko] = Some((0, 0));
        return (0, 0);
    }

    if tate + 1 == h {
        let next = if let Some(value) = memo[tate][yoko + 1] {
            value
        } else {
            saiteki(h, w, map, tate, yoko + 1, memo, !takahasi)
        };
        let now = if map[tate][yoko + 1] == '-' { -1 } else { 1 };
        memo[tate][yoko] = if takahasi {
            Some((next.0 + now, next.1))
        } else {
            Some((next.0, next.1 + now))
        };
        return memo[tate][yoko].unwrap();
    }

    if yoko + 1 == w {
        let next = saiteki(h, w, map, tate + 1, yoko, memo, !takahasi);
        let now = if map[tate + 1][yoko] == '-' { -1 } else { 1 };
        memo[tate][yoko] = if takahasi {
            Some((next.0 + now, next.1))
        } else {
            Some((next.0, next.1 + now))
        };
        return memo[tate][yoko].unwrap();
    }

    let tate_next = if let Some(old) = memo[tate + 1][yoko] {
        old
    } else {
        saiteki(h, w, map, tate + 1, yoko, memo, !takahasi)
    };
    let yoko_next = if let Some(old) = memo[tate][yoko + 1] {
        old
    } else {
        saiteki(h, w, map, tate, yoko + 1, memo, !takahasi)
    };

    let tate_now = if map[tate + 1][yoko] == '-' { -1 } else { 1 };

    let yoko_now = if map[tate][yoko + 1] == '-' { -1 } else { 1 };

    if takahasi {
        if tate_next.0 + tate_now - tate_next.1 > yoko_next.0 + yoko_now - yoko_next.1 {
            memo[tate][yoko] = Some((tate_next.0 + tate_now, tate_next.1));
        } else {
            memo[tate][yoko] = Some((yoko_next.0 + yoko_now, yoko_next.1));
        }
    } else {
        if tate_next.1 + tate_now - tate_next.0 > yoko_next.1 + yoko_now - yoko_next.0 {
            memo[tate][yoko] = Some((tate_next.0, tate_next.1 + tate_now));
        } else {
            memo[tate][yoko] = Some((yoko_next.0, yoko_next.1 + yoko_now));
        }
    }

    return memo[tate][yoko].unwrap();
}
