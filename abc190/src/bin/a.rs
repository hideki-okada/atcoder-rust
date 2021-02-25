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
    A: i32,
    B: i32,
    C: i32,
    }
    if C == 0 {
        if A <= B {
            println!("Aoki");
        } else {
            println!("Takahashi")
        }
    } else {
        if A >= B {
            println!("Takahashi");
        } else {
            println!("Aoki")
        }
    }
}
