// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 8],
    }

    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                println!("{}{}", "abcdefgh".chars().nth(j).unwrap(), 8 - i);
                return;
            }
        }
    }
}
