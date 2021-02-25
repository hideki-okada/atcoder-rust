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
        X: u32,
        Y: u32
    }
    if X <= Y {
        if X + 3 > Y {
            println!("Yes")
        } else {
            println!("No")
        }
    } else {
        if Y + 3 > X {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
