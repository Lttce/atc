// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut ans = 0;
    for bb in b {
        ans += a[bb - 1];
    }

    println!("{}", ans);
}
