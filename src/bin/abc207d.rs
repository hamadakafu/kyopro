use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::format;

use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: usize = parse_line().unwrap();
    let mut abab: Vec<(f64, f64)> = vec![];
    let mut cdcd: Vec<(f64, f64)> = vec![];
    for _ in 0..n {
        let (a, b) = parse_line().unwrap();
        abab.push((a, b));
    }
    for _ in 0..n {
        let cd = parse_line().unwrap();
        cdcd.push(cd);
    }
    // printmap(&abab);
    // println!("----------------");
    // printmap(&cdcd);
    let abjushin: (f64, f64) = {
        let (xs, ys) = abab
            .iter()
            .fold((0., 0.), |acc, (x, y)| (acc.0 + x, acc.1 + y));
        (xs as f64 / n as f64, ys as f64 / n as f64)
    };
    let cdjushin: (f64, f64) = {
        let (xs, ys) = cdcd
            .iter()
            .fold((0., 0.), |acc, (x, y)| (acc.0 + x, acc.1 + y));
        (xs as f64 / n as f64, ys as f64 / n as f64)
    };

    let abab_jushin = abab
        .into_iter()
        .map(|(x, y)| (x - abjushin.0, y - abjushin.1))
        .collect_vec();
    let cdcd_jushin = cdcd
        .into_iter()
        .map(|(x, y)| (x - cdjushin.0, y - cdjushin.1))
        .collect_vec();

    let abbase = {
        let mut tmp = (0., 0.);
        for i in 0..n {
            if abab_jushin[i] != (0., 0.) {
                tmp = abab_jushin[i];
            }
        }
        tmp
    };
    if abbase == (0., 0.) {
        println!("Yes");
        return;
    }

    for i in 0..n {
        let (cdx, cdy) = cdcd_jushin[i];
        let cos = (cdx * abbase.0 + cdy * abbase.1) / (norm(abbase) * norm((cdx, cdy)));
        let sin = (1. - cos * cos).sqrt();
        let kaitenabab = kaiten(&abab_jushin, sin, cos);
        let cdcdhas = cdcd_jushin
            .clone()
            .into_iter()
            .map(|(x, y)| {
                (
                    if &format!("{:.4}", x) == "-0.0000" {
                        -x
                    } else {
                        x
                    },
                    if &format!("{:.4}", y) == "-0.0000" {
                        -y
                    } else {
                        y
                    },
                )
            })
            .map(|(x, y)| format!("{:.4},{:.4}", x, y))
            .collect::<HashSet<String>>();
        // dbg!(&cdcdhas, &kaitenabab);
        if cdcdhas == kaitenabab {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn norm((x, y): (f64, f64)) -> f64 {
    (x * x + y * y).sqrt()
}

fn kaiten(tens: &Vec<(f64, f64)>, sin: f64, cos: f64) -> HashSet<String> {
    tens.iter()
        .map(|(x, y)| (x * cos - y * sin, x * sin + y * cos))
        .map(|(x, y)| {
            (
                if &format!("{:.4}", x) == "-0.0000" {
                    -x
                } else {
                    x
                },
                if &format!("{:.4}", y) == "-0.0000" {
                    -y
                } else {
                    y
                },
            )
        })
        .map(|(x, y)| format!("{:.4},{:.4}", x, y))
        .collect()
}
