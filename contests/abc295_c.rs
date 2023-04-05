// -*- coding:utf-8-unix -*-

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut h = HashMap::<i32, i32>::default();

    for aa in a {
        *h.entry(aa).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (_, v) in h {
        ans += v / 2
    }

    println!("{}", ans);
}
