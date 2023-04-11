// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    println!("{}", s.iter().map(|&c| if c == '0' { '1' } else { '0' }).collect::<String>());
}
