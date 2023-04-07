// -*- coding:utf-8-unix -*-

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut x: [usize; 5*n],
    }

    x.sort();

    let x = x.iter().map(|&x| x as f64).collect::<Vec<f64>>();

    println!("{}", x[n..4 * n].iter().sum::<f64>() / ((n * 3) as f64));
}
