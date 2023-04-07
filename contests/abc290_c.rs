// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let s = a.iter().collect::<std::collections::HashSet<_>>();

    let mut ans = 0;
    for i in 0..k {
        if !&s.contains(&i){
            break;
        }
        ans += 1;
    }
    println!("{}", ans);
}
