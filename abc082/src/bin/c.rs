use proconio::{fastout, input};
use std::collections::BTreeMap;
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
        A: [u32; N]
    }

    //まず数ごとにカウント
    let mut counter = BTreeMap::new();
    for a in &A {
        *counter.entry(*a).or_insert(0) += 1;
    }
    let mut answer = 0;
    for (number, count) in counter {
        answer += match number.cmp(&count) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => count,
            std::cmp::Ordering::Less => count - number,
        }
    }
    println!("{}", answer)
}
