// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }

    let mut passed = 0;
    for ss in s {
        if k <= passed {
            print!("x");
            continue;
        }
        if ss == 'o' {
            passed += 1;
        }
        print!("{}", ss);
    }
    println!();
}
