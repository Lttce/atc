// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }

    let w = w.iter().map(|s| s as &str).collect::<Vec<_>>();

    for a in w {
        match a {
            "and" | "not" | "that" | "the" | "you" => {
                println!("Yes");
                return;
            }
            _ => (),
        }
    }

    println!("No");
}
