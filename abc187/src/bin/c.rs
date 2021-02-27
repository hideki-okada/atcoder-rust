use proconio::{fastout, input};
use std::collections::HashSet;
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
        mut S: [String; N]
    }
    S.sort();
    let mut histories = HashSet::new(); // https://doc.rust-jp.rs/rust-by-example-ja/std/hash/hashset.html
    for s in S {
        if s.chars().nth(0).unwrap() == '!' {
            let s = s[1..].to_string();
            histories.insert(s);
        } else {
            if histories.contains(&s) {
                println!("{}", &s);
                return;
            }
        }
    }
    println!("satisfiable")
}
