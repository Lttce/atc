// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", s.chars().map(|c| c.to_uppercase().to_string()).collect::<String>());
}
