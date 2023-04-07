// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut m = std::collections::BTreeMap::<String, i32>::new();
    for ss in s {
        *m.entry(ss).or_insert(0) += 1
    }

    let max = &m.values().max().unwrap();
    for (k, v) in &m {
        if v == *max {
            println!("{}", k);
        }
    }
}
