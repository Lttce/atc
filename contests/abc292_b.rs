// -*- coding:utf-8-unix -*-

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        e: [(usize, usize); q],
    }

    let mut exit = vec![0; n + 1];

    for (c, x) in e {
        match c {
            1 => exit[x] += 1, // yellow
            2 => exit[x] += 2, // red
            3 => println!("{}", if 2 <= exit[x] { "Yes" } else { "No" }),
            _ => unreachable!(),
        }
    }
}
