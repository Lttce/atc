// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let a = a.iter().collect::<std::collections::HashSet<_>>();

    let mut s = Vec::<usize>::new();
    for i in 1..=n {
        if a.contains(&i) {
            &s.push(i.clone());
            continue;
        }
        print!("{} ", i);
        while let Some(x) = s.pop() {
            print!("{} ", x);
        }
    }
}
