// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u8; w]; h],
    }

    for x in a {
        for y in x {
            match y {
                0 => print!("."),
                _ => print!("{}", (('@' as u8) + y as u8) as char),
            }
        }
        println!();
    }
}
