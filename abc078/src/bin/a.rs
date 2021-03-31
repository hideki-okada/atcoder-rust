use proconio::{fastout, input};
use std::cmp::Ordering;
#[allow(unused_macros)]
macro_rules! debug {
      ($($a:expr),*) => {
          eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
      }
  }
#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: char,
        Y: char
    }
    println!(
        "{}",
        match X.cmp(&Y) {
            Ordering::Less => '<',
            Ordering::Equal => '=',
            Ordering::Greater => '>',
        }
    )
}
