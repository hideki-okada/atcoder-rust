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
        mut s: u64
    }
    let mut history = BTreeSet::new();
    for m in 1..1000000 {
        history.insert(s);
        s = f(s);
        if history.contains(&s) {
            println!("{}", m + 1);
            return;
        }
    }
}

fn f(x: u64) -> u64 {
    match x % 2 {
        0 => x / 2,
        _ => 3 * x + 1,
    }
}
