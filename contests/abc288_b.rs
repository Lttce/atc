// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }

    let s = s.into_iter().take(k).sorted().collect::<Vec<_>>();

    println!("{}", s.join("\n"));
}
