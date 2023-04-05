// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut called = vec![false; n + 1];

    for (i, v) in (1_usize..).zip(a.iter()) {
        if called[i] {
            continue;
        }
        called[*v] = true;
    }

    println!("{}", called.iter().skip(1).filter(|&&x| !x).count());

    for (i, v) in (1_usize..).zip(called.iter().skip(1)) {
        if *v {
            continue;
        }
        print!("{} ", i);
    }
}
