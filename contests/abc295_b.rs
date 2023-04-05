// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: usize,
        _: usize,
        b: [Chars; r],
    }

    // outer loop
    for (i, x) in (0_i32..).zip(b.iter()) {
        for (j, y) in (0_i32..).zip(x.iter()) {
            let mut c = y;
            // inner loop
            for (i2, x2) in (0_i32..).zip(b.iter()) {
                for (j2, y2) in (0_i32..).zip(x2.iter()) {
                    if !y2.is_ascii_digit() {
                        continue;
                    }
                    if (i - i2).abs() + (j - j2).abs() <= ((*y2 as i32) - 48) {
                        c = &'.';
                    }
                }
            }
            print!("{}", c);
        }
        println!();
    }
}
