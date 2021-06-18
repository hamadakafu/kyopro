use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (r, c): (usize, usize) = parse_line().unwrap();
    let (sy, sx): (usize, usize) = parse_line().unwrap();
    let (sy, sx) = (sy - 1, sx - 1);
    let (gy, gx): (usize, usize) = parse_line().unwrap();
    let (gy, gx) = (gy - 1, gx - 1);
    let mut map: Vec<Vec<char>> = vec![];

    for _ in 0..r {
        let line: String = parse_line().unwrap();
        map.push(line.chars().collect_vec());
    }

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((sy, sx, 0));

    let mut ans = 0;
    let mut already: HashSet<(usize, usize)> = HashSet::new();
    already.insert((sy, sx));
    while queue.len() != 0 {
        let (y, x, count) = queue.pop_front().unwrap();

        if gy == y && gx == x {
            ans = count;
            break;
        }
        if x > 0 && !already.contains(&(y, x - 1)) && map[y][x - 1] == '.' {
            queue.push_back((y, x - 1, count + 1));
            already.insert((y, x - 1));
        }
        if x + 1 != c && !already.contains(&(y, x + 1)) && map[y][x + 1] == '.' {
            queue.push_back((y, x + 1, count + 1));
            already.insert((y, x + 1));
        }
        if y > 0 && !already.contains(&(y - 1, x)) && map[y - 1][x] == '.' {
            queue.push_back((y - 1, x, count + 1));
            already.insert((y - 1, x));
        }
        if y + 1 != r && !already.contains(&(y + 1, x)) && map[y + 1][x] == '.' {
            queue.push_back((y + 1, x, count + 1));
            already.insert((y + 1, x));
        }
    }
    println!("{}", ans);
}
