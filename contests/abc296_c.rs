// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }

    let s = a
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<i32>>();

    for aa in a {
        if s.contains(&(aa + x)) {
            println!("Yes");
            return
        }
    }

    println!("No");
}
