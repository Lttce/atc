// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [i32; m],
    }

    // concat a and b
    let mut ab = a.iter().chain(b.iter()).collect::<Vec<_>>();

    ab.sort();

    for x in a.iter() {
        if let Ok(r) = ab.binary_search(&x) {
            print!("{} ", r + 1);
        }
    }
    println!();

    for x in b.iter() {
        if let Ok(r) = ab.binary_search(&x) {
            print!("{} ", r + 1);
        }
    }
    println!();
}
