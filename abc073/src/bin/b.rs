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
        N: usize,
        LR: [(u32, u32);N]
    }
    println!("{}", LR.iter().fold(0, |sum, lr| sum + (lr.1 - lr.0 + 1)));
}
