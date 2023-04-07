// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut m = std::collections::HashMap::<String, i32>::new();
    for ss in s {
        *m.entry(ss).or_insert(0) += 1
    }

    let max = &m.values().max().unwrap();
    let mut ans = vec![];
    for mm in &m {
        if mm.1 == *max {
            ans.push(mm.0.clone())
        }
    }
    ans.sort();
    println!("{}", ans.join("\n"));
}
