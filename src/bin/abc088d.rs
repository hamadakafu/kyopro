use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (h, w): (usize, usize) = parse_line().unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let line: String = parse_line().unwrap();
        map.push(line.chars().collect_vec());
    }

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((0, 0, 0));
    let mut already: Vec<Vec<bool>> = vec![vec![false; w]; h];
    already[0][0] = true;
    let mut ans = 0;
    while !queue.is_empty() {
        let (si, sj, now) = queue.pop_front().unwrap();
        if si == h - 1 && sj == w - 1 {
            ans = now;
            break;
        }

        if si > 0 && !already[si - 1][sj] && map[si - 1][sj] == '.' {
            queue.push_back((si - 1, sj, now + 1));
            already[si - 1][sj] = true;
        }
        if sj > 0 && !already[si][sj - 1] && map[si][sj - 1] == '.' {
            queue.push_back((si, sj - 1, now + 1));
            already[si][sj - 1] = true;
        }
        if si < h - 1 && !already[si + 1][sj] && map[si + 1][sj] == '.' {
            queue.push_back((si + 1, sj, now + 1));
            already[si + 1][sj] = true;
        }
        if sj < w - 1 && !already[si][sj + 1] && map[si][sj + 1] == '.' {
            queue.push_back((si, sj + 1, now + 1));
            already[si][sj + 1] = true;
        }
    }
    let mut kabe = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '#' {
                kabe += 1;
            }
        }
    }
    if ans == 0 {
        println!("{}", -1);
    } else {
        println!("{}", h * w - 2 - (ans - 1) - kabe);
    }
}
