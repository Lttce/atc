// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    let mut ans = 1;
    for i in 2..x {
        let mut a = i * i;
        while a <= x {
            ans = ans.max(a);
            a *= i;
        }
    }
    println!("{}", ans);
}
