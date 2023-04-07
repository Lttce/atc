// -*- coding:utf-8-unix -*-
 
use proconio::fastout;
use proconio::input;
 
#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
    }
 
    println!("{}", n * n - a);
}