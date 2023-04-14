// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: i32,
        b: [i32; m],
        a: [[i32; m]; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let mut sum = 0;
        for j in 0..m {
            sum += a[i][j] * b[j];
        }
        if sum + c > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
