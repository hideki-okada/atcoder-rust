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
        A: u32,
        B: u32,
        C: u32,
        D: u32
    }
    if A + B < C + D {
        println!("Right")
    } else if A + B == C + D {
        println!("Balanced")
    } else {
        println!("Left")
    }
}
