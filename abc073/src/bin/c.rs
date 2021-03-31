use proconio::{fastout, input};
use std::collections::BTreeSet;
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
        A: [u64; N]
    }
    let mut paper = BTreeSet::new();
    for a in A {
        if paper.contains(&a) {
            paper.remove(&a);
        } else {
            paper.insert(a);
        }
    }
    println!("{}", paper.len())
}
