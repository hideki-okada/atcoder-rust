use itertools::Itertools;
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
        AB: [(u32, u32); N]
    }
    let A = AB.iter().map(|&o| o.0).sorted().collect_vec();
    let B = AB.iter().map(|&o| o.1).sorted().collect_vec();
}
