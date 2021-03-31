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
        B: u32
    }
    if A + B >= 15 && B >= 8 {
        println!("{}", 1);
    } else if A + B >= 10 && B >= 3 {
        println!("{}", 2);
    } else if A + B >= 3 {
        println!("{}", 3);
    } else {
        println!("{}", 4);
    }
}
