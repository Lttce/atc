// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // saved position
    let mut pos = std::collections::HashSet::<(i32, i32)>::with_capacity(n);
    pos.insert((0, 0));

    let mut xy = (0, 0);
    for c in s {
        match c {
            'R' => xy.0 += 1,
            'L' => xy.0 -= 1,
            'U' => xy.1 += 1,
            'D' => xy.1 -= 1,
            _ => unreachable!(),
        }

        // check if the position is already saved
        if pos.contains(&xy) {
            println!("Yes");
            return;
        }

        // save the position
        pos.insert(xy);
    }
    println!("No");
}
