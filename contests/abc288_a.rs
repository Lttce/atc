// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(i32, i32); n],
    }

    for (a, b) in ab {
        println!("{}", a + b);
    }
}
