use proconio::marker::Chars;
use proconio::{fastout, input};
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
    C: Chars
    }
    if C[0] == C[1] && C[1] == C[2] {
        println!("Won");
    } else {
        println!("Lost");
    }
}
