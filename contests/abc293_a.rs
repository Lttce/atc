// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    for x in s.chunks(2) {
        print!("{}{}", x[1], x[0]);
    }
    println!()
}
